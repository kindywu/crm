use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    sender_email: String,
    user_stats: String,
    metadata: String,
    notification: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from  ./app.yml, or /etc/config/app.yml, or from env CHAT_CONFIG
        // println!("{:?}", env::current_dir());
        let ret = match (
            File::open("crm.yml"),
            File::open("/etc/config/crm.yml"),
            env::var("CRM_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}
