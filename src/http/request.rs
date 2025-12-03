use super::{Method, ParseError};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str;

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

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        // Convert bytes to UTF-8 string
        let request_str = str::from_utf8(buffer)
            .map_err(|_| ParseError::InvalidEncoding)?;
        
        // Placeholder for parsing logic (will implement in next commit)
        // For now, return an error
        Err(ParseError::InvalidRequest)
    }
}
