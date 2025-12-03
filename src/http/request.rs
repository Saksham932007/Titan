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
        
        // Split request into lines
        let mut lines = request_str.lines();
        
        // Parse the request line (e.g., "GET /path HTTP/1.1")
        let request_line = lines.next().ok_or(ParseError::InvalidRequest)?;
        let mut parts = request_line.split_whitespace();
        
        // Extract method
        let method_str = parts.next().ok_or(ParseError::InvalidRequest)?;
        let method = method_str.parse::<Method>()
            .map_err(|_| ParseError::InvalidMethod)?;
        
        // Extract path and query
        let uri = parts.next().ok_or(ParseError::InvalidRequest)?;
        let (path, query) = if let Some(query_idx) = uri.find('?') {
            let (p, q) = uri.split_at(query_idx);
            (p.to_string(), Some(q[1..].to_string()))
        } else {
            (uri.to_string(), None)
        };
        
        // Verify HTTP protocol
        let protocol = parts.next().ok_or(ParseError::InvalidRequest)?;
        if !protocol.starts_with("HTTP/") {
            return Err(ParseError::InvalidProtocol);
        }
        
        // Parse headers
        let mut headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                break; // End of headers
            }
            if let Some(colon_idx) = line.find(':') {
                let (key, value) = line.split_at(colon_idx);
                headers.insert(
                    key.trim().to_string(),
                    value[1..].trim().to_string(),
                );
            }
        }
        
        Ok(Request {
            path,
            query,
            method,
            headers,
        })
    }
}
