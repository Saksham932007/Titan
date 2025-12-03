
pub mod config;
pub mod http;
pub mod server;

pub use config::Config;
pub use server::Server;

pub fn run() {
    let cfg = Config::from_env();
    println!("Titan library initialized on {}:{}", cfg.ip, cfg.port);
}
