use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn new(
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            method: method,
            url: url,
            headers: headers,
            body: body,
        }
    }
}
