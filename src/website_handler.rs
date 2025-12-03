use crate::http::{Handler, Method, Request, Response, StatusCode};
use std::path::PathBuf;

pub struct WebsiteHandler {
    public_path: PathBuf,
}

impl WebsiteHandler {
    pub fn new(public_path: PathBuf) -> Self {
        WebsiteHandler { public_path }
    }

    fn map_path(&self, request_path: &str) -> PathBuf {
        // Map request path to file system path
        let mut path = self.public_path.clone();
        
        // Remove leading slash and append to public path
        let sanitized = request_path.trim_start_matches('/');
        
        // Default to index.html for root path
        if sanitized.is_empty() {
            path.push("index.html");
        } else {
            path.push(sanitized);
        }
        
        path
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method {
            Method::GET => {
                let file_path = self.map_path(&request.path);
                
                // Placeholder: file reading will be added in next commit
                Response::new(
                    StatusCode::Ok,
                    Some(format!("Would serve: {:?}", file_path)),
                )
            }
            _ => Response::new(
                StatusCode::NotFound,
                Some("Method not supported".to_string()),
            ),
        }
    }
}
