use tokio::io::{BufReader, AsyncRead};
use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub async fn build<R: AsyncRead>(reader: &BufReader<R>) -> Request {
        Request {
            method: HttpMethod::GET,
            target: "derp".to_string(),
            version: HttpVersion::OneOh,
            headers: HashMap::from([("herp".to_string(), "derp".to_string())])
        }
    }
}

pub fn build_request_bytes(request_line: &str, headers: Vec<(&str, &str)>) -> Vec<u8> {
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
    use tokio::io::BufReader;
    use std::io::Cursor;
    
    #[test]
    fn test_build_request_bytes() {
        let built = build_request_bytes("GET / HTTP/1.0", [
            ("Connection", "close"),
            ("User-Agent", "rustlang test suite"),
            ("Pragma", "no-cache"),
            ("Host", "rustlang.test.host.example.com:1234")
        ].to_vec());

        let manual = b"GET / HTTP/1.0\r\nConnection: close\r\nUser-Agent: rustlang test suite\r\nPragma: no-cache\r\nHost: rustlang.test.host.example.com:1234".to_vec();

        assert_eq!(String::from_utf8(built), String::from_utf8(manual))
    }
    
    #[tokio::test]
    async fn http_get_http10_1() {
        let sample = build_request_bytes("GET / HTTP/1.0", [
            ("Connection", "close"),
            ("User-Agent", "rustlang test suite"),
            ("Pragma", "no-cache"),
            ("Host", "rustlang.test.host.example.com:1234")
        ].to_vec());

        let mut reader = BufReader::new(Cursor::new(sample));

        let request = Request::build(&mut reader).await;

        // TODO: manually build request object for comparison
        assert!(true)
    }
}