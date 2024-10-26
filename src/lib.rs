use std::{collections::HashMap, str::FromStr};

use pyo3::{exceptions::{PyRuntimeError, PyTypeError}, prelude::*, types::{PyList, PyType}, wrap_pyfunction};
mod request;
use reqwest;
use request::{Request, Response};

use tokio;
#[pyfunction]
#[pyo3(signature = (requests, return_panic=false, proxy=Vec::new(), /))]
fn batch_request<'a>(py: Python<'a>, requests: &Bound<'a, PyList>, return_panic: bool, proxy: Vec<(String, String)>) -> PyResult<Bound<'a, PyAny>> {
    for request in requests.iter() {
        if !request.is_instance_of::<Request>() {
            return Err(PyTypeError::new_err("Invalid request type"));
        }
    }
    let mut client = reqwest::Client::builder();
    for (key, value) in proxy.iter() {
        match key.as_str() {
            "http" => client = client.proxy(reqwest::Proxy::http(value).unwrap()),
            "https" => client = client.proxy(reqwest::Proxy::https(value).unwrap()),
            _ => return Err(PyTypeError::new_err("Invalid proxy type")),
        };
    }
    let client = client.build().unwrap();
    let mut features = Vec::with_capacity(requests.len());
    for request in requests.iter() {
        let req = Request::extract_bound(&request).unwrap();
        let mut builder = client.request(reqwest::Method::from_bytes(req.method.as_bytes()).unwrap(), reqwest::Url::from_str(&req.url).unwrap());
        if let Some(body) = req.body {
            builder = builder.body(body);
        }
        for (header, value) in req.headers.iter() {
            builder = builder.header(header, value);
        }
        features.push(pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            let response = builder.send().await;
            match response {
                Ok(response) => {
                    Ok(Response::new(
                        response.status().as_u16(),
                        response.headers().into_iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect(),
                        response.bytes().await?.to_vec()
                    ))
                }
                Err(e) => {
                    Err(e)
                }
            } 
        }));
    }
    return pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let mut res = Vec::with_capacity(features.len());
        for f in features {
            res.push(f.await.unwrap());
        }
        
        Python::with_gil(|py| -> Result<Vec<Py<pyo3::PyAny>>, PyErr> {
            let mut ret = Vec::with_capacity(res.len());
            for r in res {
                match r {
                    Ok(response) => {
                        let pyobj = Py::new(py, response);
                        
                        ret.push(pyobj.unwrap().to_object(py));
                    },
                    Err(e) => {
                        let panic = PyRuntimeError::new_err(e.to_string());
                        if return_panic {
                            ret.push(panic.to_object(py));
                        } else {
                            return Err(panic);
                        }
                    }
                };
            }
            Ok(ret)
        })
    });
    
    
}

#[pymodule]
#[pyo3(name = "batch_http_request")]
fn batch_http_request(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(batch_request, m)?)?;
    m.add_class::<Request>()?;
    m.add_class::<Response>()?;
    Ok(())
}