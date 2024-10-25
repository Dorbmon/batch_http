use pyo3::{prelude::*, types::PyList, wrap_pyfunction};

use pyo3::prelude::*;

#[pyclass(set_all, get_all)]
#[derive(FromPyObject)]
#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[pymethods]
impl Request {
    #[new]
    #[pyo3(signature = (url, method, headers=vec![], body=None))]
    fn new(url: String, method: String, headers: Vec<(String, String)>, body: Option<Vec<u8>>) -> Self {
        Request {
            url,
            method,
            headers,
            body,
        }
    }
}

#[pyclass(set_all, get_all)]
#[derive(FromPyObject)]
pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
#[pymethods]
impl Response {
    #[new]
    #[pyo3(signature = (status_code, headers, body))]
    pub fn new(status_code: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        Response {
            status_code,
            headers,
            body,
        }
    }
}