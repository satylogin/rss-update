use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) type ConfigList = Vec<Config>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Returns path where config should reside.
fn config_path() -> String {
    let config_path = Path::new(&crate::base_dir()).join("config.json");
    String::from(config_path.to_str().unwrap())
}

/// Feed Configuration used to track feed status.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Config {
    pub(crate) feed: String,
    pub(crate) updated: Option<DateTime<Utc>>,
}

/// Reads configs from path and parse it as `ConfigList`.
pub(crate) fn get() -> Result<ConfigList> {
    _get(&config_path())
}

fn _get(path: &str) -> Result<ConfigList> {
    let config = fs::read_to_string(path)?;
    Ok(serde_json::from_str(config.as_str())?)
}

pub(crate) fn replace(configs: ConfigList) -> Result<ConfigList> {
    let data = serde_json::to_string_pretty(&configs)?;
    fs::write(config_path(), data)?;
    Ok(configs)
}

pub(crate) fn setup() -> Result<()> {
    let config_path = config_path();
    if Path::new(&config_path).is_file() {
        println!("config file already exists.");
    } else {
        println!("creating config path.");
        fs::write(config_path, "[]")?;
    }
    Ok(())
}

pub(crate) fn update(config: Config) -> Result<ConfigList> {
    replace(_update(get()?, config))
}

fn _update(mut configs: ConfigList, config: Config) -> ConfigList {
    for c in &configs {
        if c.feed == config.feed {
            println!(
                "feed: {} is already being tracked. skipping re-adding",
                &config.feed
            );
            return configs;
        }
    }
    println!("adding feed: {} for tracking", &config.feed);
    configs.push(config);
    configs
}

pub(crate) fn remove(feed: &str) -> Result<ConfigList> {
    let configs = get()?.into_iter().filter(|c| c.feed != feed).collect();
    replace(configs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_path() {
        assert!(config_path().starts_with('/'));
    }

    #[test]
    fn test_get() {
        let mut file = NamedTempFile::new().unwrap();
        let feed1 = "https://satylogin.medium.com/feed".to_string();
        let feed2 = "https://motw.rs/rss.xml".to_string();
        let now = chrono::Utc::now();
        let content = format!(
            r#"[
            {{
                "feed": "{feed1}",
                "updated": "{now}"
            }},
            {{
                "feed": "{feed2}"
            }}
        ]"#,
            feed1 = feed1,
            now = now,
            feed2 = feed2
        );
        writeln!(file, "{}", content).unwrap();

        let expected = vec![
            Config {
                feed: feed1,
                updated: Some(now),
            },
            Config {
                feed: feed2,
                updated: None,
            },
        ];
        let output = _get(file.path().to_str().unwrap()).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn test_update_new_config() {
        let configs = vec![
            Config {
                feed: "feed1".to_string(),
                updated: None,
            },
            Config {
                feed: "feed3".to_string(),
                updated: None,
            },
        ];
        let config = Config {
            feed: "feed2".to_string(),
            updated: None,
        };
        let updated = _update(configs, config);
        let feeds = updated
            .iter()
            .map(|c| c.feed.to_string())
            .collect::<Vec<_>>();
        assert_eq!(3, feeds.len());
        for feed in ["feed1", "feed2", "feed3"] {
            assert!(feeds.contains(&String::from(feed)));
        }
    }

    #[test]
    fn test_update_existing_config() {
        let configs = vec![
            Config {
                feed: "feed1".to_string(),
                updated: None,
            },
            Config {
                feed: "feed3".to_string(),
                updated: None,
            },
        ];
        let config = Config {
            feed: "feed3".to_string(),
            updated: None,
        };
        let updated = _update(configs, config);
        let feeds = updated
            .iter()
            .map(|c| c.feed.to_string())
            .collect::<Vec<_>>();
        assert_eq!(2, feeds.len());
        for feed in ["feed1", "feed3"] {
            assert!(feeds.contains(&String::from(feed)));
        }
    }
}
