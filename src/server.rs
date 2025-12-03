use crate::Config;
use std::io::{Read, Result as IoResult};
use std::net::TcpListener;

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
        
        for stream in listener.incoming() {
            let mut stream = stream?;
            
            // Read bytes into a 1024-byte buffer
            let mut buffer = [0; 1024];
            stream.read(&mut buffer)?;
            
            println!("Received {} bytes", buffer.len());
        }
        
        Ok(())
    }
}
