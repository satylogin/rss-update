pub(crate) mod config;

use config::Config;
use rss::Channel;
use std::collections::HashMap;
use std::error::Error;

async fn rfc_channel(url: &str) -> Result<Channel, Box<dyn Error>> {
    let data = reqwest::get(url).await?.bytes().await?;
    Ok(Channel::read_from(&data[..])?)
}

async fn new_feeds(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
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

    let mut feeds_to_read = HashMap::new();
    for config in configs {
        let new_feeds = new_feeds(&config.feed).await?;
        feeds_to_read.insert(config.feed, new_feeds);
    }
    dbg!(feeds_to_read);

    Ok(())
}
