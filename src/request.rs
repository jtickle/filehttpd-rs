use tokio::io::{BufReader, AsyncRead};
use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
}

macro_rules! header {
    ($k:expr, $v:expr) => (($k.to_string(), $v.to_string()))
}

impl Request {
    pub fn new(method: HttpMethod, target: String, version: HttpVersion, headers: HashMap<String, String>) -> Request {
        Request { method, target, version, headers }
    }
    pub async fn build<R: AsyncRead>(reader: &BufReader<R>) -> Request {
        Request {
            method: HttpMethod::GET,
            target: "derp".to_string(),
            version: HttpVersion::OneOh,
            headers: HashMap::from([
                header!("herp", "derp")
            ]),
        }
    }
}

pub fn build_request_bytes(request_line: &str, headers: Vec<(String, String)>) -> Vec<u8> {
    [
        request_line.as_bytes(),
        b"\r\n",
        headers
            .iter()
            .map(|(k,v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n")
            .as_bytes()
    ].concat()
}

#[cfg(test)]
mod tests {
    use crate::request::{Request, build_request_bytes};
    use std::collections::HashMap;
    use tokio::io::BufReader;
    use std::io::Cursor;
    use crate::*;
    
    #[test]
    fn test_build_request_bytes() {
        let built = build_request_bytes("GET / HTTP/1.0", [
            header!("Connection", "close"),
            header!("User-Agent", "rustlang test suite"),
            header!("Pragma", "no-cache"),
            header!("Host", "rustlang.test.host.example.com:1234")
        ].to_vec());

        let manual = b"GET / HTTP/1.0\r\nConnection: close\r\nUser-Agent: rustlang test suite\r\nPragma: no-cache\r\nHost: rustlang.test.host.example.com:1234".to_vec();

        assert_eq!(String::from_utf8(built), String::from_utf8(manual))
    }
    
    #[tokio::test]
    async fn http_get_http10_1() {
        let sample = build_request_bytes("GET / HTTP/1.0", [
            header!("Connection", "close"),
            header!("User-Agent", "rustlang test suite"),
            header!("Pragma", "no-cache"),
            header!("Host", "rustlang.test.host.example.com:1234")
        ].to_vec());

        let expected = Request {
            method: HttpMethod::GET,
            target: "/".to_string(),
            version: HttpVersion::OneOh,
            headers: HashMap::from([
                header!("Connection", "close"),
            ]),
        };

        let mut reader = BufReader::new(Cursor::new(sample));
        let result = Request::build(&mut reader).await;

        assert_eq!(expected, result)
    }
}