pub(crate) mod config;

use chrono::{DateTime, Utc};
use config::Config;
use rss::Channel;
use std::collections::HashMap;
use std::error::Error;

async fn rfc_channel(url: String) -> Result<Channel, Box<dyn Error>> {
    let data = reqwest::get(url.as_str()).await?.bytes().await?;
    Ok(Channel::read_from(&data[..])?)
}

async fn new_feeds(url: String) -> Result<Vec<String>, Box<dyn Error>> {
    let channel = rfc_channel(url).await?;
    let mut links = vec![];
    for item in channel.items().iter() {
        links.push(item.link().unwrap().to_owned());
    }

    Ok(links)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configs: Vec<Config> = config::feed_config()?;

    let current_time: DateTime<Utc> = Utc::now();
    let mut feeds_to_read = HashMap::new();

    let mut feeds_futures = HashMap::new();
    for config in configs.clone() {
        let feeds_future = new_feeds(config.feed.clone());
        feeds_futures.insert(config.feed.clone(), feeds_future);
    }
    let mut new_config = vec![];
    for (feed, future) in feeds_futures {
        let to_read = future.await?;
        feeds_to_read.insert(feed.clone(), to_read);
        new_config.push(Config {
            feed,
            updated: Some(current_time),
        });
    }
    dbg!(feeds_to_read, new_config);

    Ok(())
}
