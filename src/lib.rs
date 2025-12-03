
pub mod config;
pub mod http;

pub use config::Config;

pub fn run() {
    let cfg = Config::from_env();
    println!("Titan library initialized on {}:{}", cfg.ip, cfg.port);
}
