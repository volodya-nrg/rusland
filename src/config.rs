use config::{Config as ConfigExternal, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub service_name: String,
    pub version: String,
    pub sqlite_filepath: String,
    pub http_server: HttpServer,
    pub log: Log,
}
#[derive(Deserialize)]
pub struct HttpServer {
    pub address: String,
}
#[derive(Deserialize)]
pub struct Log {
    pub level: String,
}

pub fn new(filepath: &str) -> Result<Config, ConfigError> {
    let cfg: Config = ConfigExternal::builder()
        .add_source(File::new(filepath, FileFormat::Yaml))
        .build()?
        .try_deserialize()?;
    Ok(cfg)
}
