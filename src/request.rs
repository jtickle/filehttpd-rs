use tokio::io::BufReader;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: Map<String, String>,
}

impl Request {
    pub async fn build(reader: &BufReader) {

    }
}