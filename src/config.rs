use config::ConfigError;

#[derive(serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        cfg.try_deserialize()
    }
}
