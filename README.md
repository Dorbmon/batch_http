# Batch HTTP Request

## Overview

The Batch HTTP Request is a Python package designed to handle multiple HTTP requests efficiently. It allows users to send a batch of requests and receive the corresponding responses in a structured format. The library also provides error handling capabilities, returning errors alongside successful responses if desired.

## Features

- Send multiple HTTP requests in a single batch.
- Supports various HTTP methods (GET, POST, etc.).
- Handles headers and request bodies.
- Optionally returns errors encountered during request processing.

## Installation

To install the library, you can use pip:

```bash
pip install batch-http-request
```

## Usage

### Classes

#### Request

The `Request` class is used to define individual HTTP requests.

- **Attributes:**
  - `url` (str): The URL for the request.
  - `method` (str): The HTTP method (e.g., "GET", "POST").
  - `headers` (List[Tuple[str, str]]): Optional list of headers.
  - `body` (Optional[bytes]): Optional request body.

- **Constructor:**

  ```python
  Request(url: str, method: str, headers: Optional[List[Tuple[str, str]]] = [], body: Optional[bytes] = None)
  ```

#### Response

The `Response` class represents the HTTP response.

- **Attributes:**
  - `status_code` (int): The HTTP status code.
  - `headers` (List[Tuple[str, str]]): List of response headers.
  - `body` (bytes): Response body.

- **Constructor:**

  ```python
  Response(status_code: int, headers: List[Tuple[str, str]], body: bytes)
  ```

### Function

#### batch_request

The `batch_request` function processes a list of `Request` objects and returns their corresponding `Response` objects or errors.

- **Parameters:**
  - `requests` (List[Request]): A list of `Request` objects to be processed.
  - `return_panic` (bool): If set to `True`, the function returns errors encountered during the request processing as `RuntimeError` objects. Defaults to `False`.

- **Returns:**
  - List[Union[Response, RuntimeError]]: A list containing `Response` objects for successful requests and `RuntimeError` objects for failed requests if `return_panic` is `True`.

- **Example:**

  ```python
  from batch_http_request import Request, batch_request

  requests = [
      Request(url="https://api.example.com/data", method="GET"),
      Request(url="https://api.example.com/submit", method="POST", body=b'{"key": "value"}')
  ]

  responses = batch_request(requests, return_panic=True)

  for response in responses:
      if isinstance(response, Response):
          print(f"Status: {response.status_code}, Body: {response.body}")
      else:
          print(f"Error: {response}")
  ```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact

For any questions or issues, please open an issue on the GitHub repository or contact the maintainers directly.