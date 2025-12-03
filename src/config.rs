use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

impl Config {
    /// Load configuration from environment variables, with sensible defaults.
    pub fn from_env() -> Self {
        let _ = dotenv::dotenv();

        let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(80);

        Config { ip, port }
    }
}
