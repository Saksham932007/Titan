
pub mod config;
pub mod http;
pub mod server;
pub mod thread_pool;
pub mod website_handler;

pub use config::Config;
pub use server::Server;
pub use website_handler::WebsiteHandler;

pub fn run() {
    let cfg = Config::from_env();
    let server = Server::new(cfg);
    
    if let Err(e) = server.run() {
        eprintln!("Server error: {}", e);
    }
}
