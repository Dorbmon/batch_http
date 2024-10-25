from typing import List, Tuple, Optional, Union

class Request:
    url: str
    method: str
    headers: List[Tuple[str, str]]
    body: Optional[bytes]

    def __init__(self, url: str, method: str, headers: Optional[List[Tuple[str, str]]] = [], body: Optional[bytes] = None) -> None: ...

class Response:
    status_code: int
    headers: List[Tuple[str, str]]
    body: bytes

    def __init__(self, status_code: int, headers: List[Tuple[str, str]], body: bytes) -> None: ...

def batch_request(requests: List[Request], return_panic: bool = False) -> List[Union[Response, RuntimeError]]: ...
