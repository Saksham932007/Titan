use super::Method;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub query: Option<String>,
    pub method: Method,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(
        method: Method,
        path: String,
        query: Option<String>,
        headers: HashMap<String, String>,
    ) -> Self {
        Request {
            path,
            query,
            method,
            headers,
        }
    }
}
