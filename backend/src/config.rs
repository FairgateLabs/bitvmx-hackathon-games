use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
    pub bitvmx: BitVMXClientConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_headers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitVMXClientConfig {
    pub broker_port: u16,
}

impl Config {
    pub fn load(name: &str) -> Result<Self, anyhow::Error> {
        let config = config::Config::builder()
            .add_source(config::File::with_name(&format!("config/{name}.yaml")))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        let config: Config = config.try_deserialize()?;
        Ok(config)
    }

    pub fn server_addr(&self) -> Result<SocketAddr, anyhow::Error> {
        let addr = format!("{}:{}", self.server.host, self.server.port);
        let socket_addr = addr.parse()?;
        Ok(socket_addr)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            logging: LoggingConfig {
                level: "debug".to_string(),
            },
            cors: CorsConfig {
                allowed_origins: vec!["*".to_string()],
                allowed_headers: vec!["*".to_string()],
            },
            bitvmx: BitVMXClientConfig { broker_port: 22222 },
        }
    }
}
