use crate::Config;
use std::io::Result as IoResult;
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
        
        // Placeholder: accept connections loop will be added in next commit
        Ok(())
    }
}
