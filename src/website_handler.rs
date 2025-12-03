use crate::http::{Handler, Method, Request, Response, StatusCode};
use std::fs;
use std::path::PathBuf;

pub struct WebsiteHandler {
    public_path: PathBuf,
}

impl WebsiteHandler {
    pub fn new(public_path: PathBuf) -> Self {
        WebsiteHandler { public_path }
    }

    fn map_path(&self, request_path: &str) -> Option<PathBuf> {
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
        
        // SECURITY: Canonicalize paths and ensure we stay within public_path
        match path.canonicalize() {
            Ok(canonical_path) => {
                // Get the canonical version of public_path
                if let Ok(canonical_public) = self.public_path.canonicalize() {
                    // Ensure the requested path starts with the public directory
                    if canonical_path.starts_with(&canonical_public) {
                        return Some(canonical_path);
                    }
                }
                None // Path traversal attempt detected
            }
            Err(_) => None, // Path doesn't exist or can't be canonicalized
        }
    }

    fn read_file(&self, path: &PathBuf) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method {
            Method::GET => {
                match self.map_path(&request.path) {
                    Some(file_path) => match self.read_file(&file_path) {
                        Ok(contents) => Response::new(StatusCode::Ok, Some(contents)),
                        Err(_) => Response::new(
                            StatusCode::NotFound,
                            Some("File not found".to_string()),
                        ),
                    },
                    None => {
                        // Path traversal attempt or invalid path
                        Response::new(
                            StatusCode::NotFound,
                            Some("Invalid path".to_string()),
                        )
                    }
                }
            }
            _ => Response::new(
                StatusCode::NotFound,
                Some("Method not supported".to_string()),
            ),
        }
    }
}
