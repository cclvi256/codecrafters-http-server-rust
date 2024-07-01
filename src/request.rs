use std::{collections::HashMap, fmt::{Display, Formatter, Result}};

pub struct Request {
    pub method: String,
    pub url: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(
        method: &str,
        url: &str,
        version: &str,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Self {
        Self {
            method: method.to_string(),
            url: url.to_string(),
            version: version.to_string(),
            headers,
            body,
        }
    }

    pub fn new_from_string(request: String) -> Self {
        let mut parts = request.splitn(2, "\r\n\r\n");
        
        let head = parts.next().unwrap();
        
        let mut lines = head.lines();
        
        let request_line = lines.next().unwrap();
        let mut parts = request_line.split_whitespace();
        
        let method = parts.next().unwrap();
        let url = parts.next().unwrap();
        let version = parts.next().unwrap();
        
        let mut headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            headers.insert(key.to_string(), value.to_string());
        }
        
        let body = parts.next().unwrap().as_bytes().to_vec();
        
        Self::new(method, url, version, headers, body)
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut request = format!("{} {} {}\r\n", self.method, self.url, self.version);
        
        for (key, value) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        request.push_str("\r\n");
        
        request.push_str(&String::from_utf8_lossy(&self.body));
        
        write!(f, "{}", request)
    }
}
