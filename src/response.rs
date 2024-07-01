use std::{collections::HashMap, fmt::{Display, Formatter, Result}};

pub struct Response {
    pub code: u16,
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(code: u16, status: &str, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self {
            code,
            status: status.to_string(),
            headers,
            body,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut res = format!("HTTP/1.1 {} {}\r\n", self.code, self.status);
        for (key, value) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", key, value));
        }
        res.push_str("\r\n");
        res.push_str(&String::from_utf8_lossy(&self.body));

        write!(f, "{}", res)
    }
}
