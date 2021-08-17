use crate::config::Config;
use chrono::{DateTime, FixedOffset, Utc};
use futures::future;
use rss::Channel;
use std::collections::HashMap;
use std::error::Error;

async fn rfc_channel(url: String) -> Result<Channel, Box<dyn Error>> {
    let data = reqwest::get(url.as_str()).await?.bytes().await?;
    Ok(Channel::read_from(&data[..])?)
}

fn date_time(dt: &str) -> Result<DateTime<FixedOffset>, Box<dyn Error>> {
    Ok(DateTime::parse_from_rfc2822(dt)?)
}

async fn new_feeds(
    url: String,
    updated: Option<DateTime<Utc>>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let channel = rfc_channel(url).await?;
    let updated = updated.unwrap_or(Utc::now());
    let links = channel
        .items()
        .into_iter()
        .filter(|item| date_time(item.pub_date().unwrap()).ok().unwrap() > updated)
        .map(|item| String::from(item.link().unwrap()))
        .collect::<Vec<_>>();
    Ok(links)
}

#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) feeds: HashMap<String, Vec<String>>,
    pub(crate) config: Vec<Config>,
}

pub(crate) async fn feeds_and_config(configs: Vec<Config>) -> Result<Context, Box<dyn Error>> {
    let mut feeds_futures = vec![];
    for config in configs.iter() {
        feeds_futures.push(new_feeds(config.feed.clone(), config.updated.clone()));
    }
    let new_feeds = future::join_all(feeds_futures).await;

    let current_time: DateTime<Utc> = Utc::now();
    let mut new_config = vec![];
    let mut feeds_to_read = HashMap::new();
    for (config, to_read) in configs.into_iter().zip(new_feeds.into_iter()) {
        feeds_to_read.insert(config.feed.clone(), to_read?);
        new_config.push(Config {
            updated: Some(current_time),
            ..config
        })
    }

    Ok(Context {
        feeds: feeds_to_read,
        config: new_config,
    })
}