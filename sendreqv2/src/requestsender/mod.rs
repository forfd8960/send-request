use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Sender {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: String,
}
