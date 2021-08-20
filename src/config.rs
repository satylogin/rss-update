use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn config_path() -> String {
    let config_path = Path::new(&crate::base_dir()).join("config.json");
    String::from(config_path.to_str().unwrap())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Config {
    pub(crate) feed: String,
    pub(crate) updated: Option<DateTime<Utc>>,
}

pub(crate) fn get() -> Result<Vec<Config>, Box<dyn Error>> {
    let config = fs::read_to_string(config_path())?;
    Ok(serde_json::from_str(config.as_str())?)
}

pub(crate) fn update(configs: Vec<Config>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(&configs)?;
    fs::write(config_path(), data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path() {
        assert!(config_path().starts_with("/"));
    }
}
