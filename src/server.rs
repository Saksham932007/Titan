use crate::http::{Handler, Request, Response, StatusCode};
use crate::logger;
use crate::thread_pool::ThreadPool;
use crate::{Config, WebsiteHandler};
use std::convert::TryFrom;
use std::io::{Read, Result as IoResult, Write};
use std::net::TcpListener;
use std::path::PathBuf;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let address = format!("{}:{}", config.ip, config.port);
        Server { address }
    }

    pub fn run(&self) -> IoResult<()> {
        println!("Starting server on {}", self.address);
        let listener = TcpListener::bind(&self.address)?;
        
        println!("Server listening on {}", self.address);
        
        // Create thread pool with 4 workers
        let pool = ThreadPool::new(4);
        
        for stream in listener.incoming() {
            let mut stream = stream?;
            
            // Dispatch connection handling to thread pool
            pool.execute(move || {
                let mut buffer = [0; 1024];
                
                if let Err(e) = stream.read(&mut buffer) {
                    eprintln!("Failed to read from stream: {}", e);
                    return;
                }
                
                // Parse buffer into Request
                match Request::try_from(&buffer[..]) {
                    Ok(request) => {
                        // Create handler and process request
                        let mut handler = WebsiteHandler::new(PathBuf::from("public"));
                        let response = handler.handle_request(&request);
                        
                        // Log the request
                        logger::log_request(
                            &request.method.to_string(),
                            &request.path,
                            200, // Simplified: using 200 for now
                        );
                        
                        // Write response to stream
                        if let Err(e) = response.write_to(&mut stream) {
                            eprintln!("Failed to write response: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse request: {}", e);
                        
                        // Send error response
                        let error_response = Response::new(
                            StatusCode::BadRequest,
                            Some("Bad Request".to_string()),
                        );
                        let _ = error_response.write_to(&mut stream);
                    }
                }
            });
        }
        
        Ok(())
    }
}
