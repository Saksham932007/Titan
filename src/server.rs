use crate::Config;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let address = format!("{}:{}", config.ip, config.port);
        Server { address }
    }
}
