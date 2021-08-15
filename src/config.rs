use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

const CONFIG_PATH: &str = "data/config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Config {
    pub(crate) feed: String,
    pub(crate) updated: Option<DateTime<Utc>>,
}

pub(crate) fn feed_config() -> Result<Vec<Config>, Box<dyn Error>> {
    let config = fs::read_to_string(CONFIG_PATH)?;
    Ok(serde_json::from_str(config.as_str())?)
}
