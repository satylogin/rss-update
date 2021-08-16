pub(crate) mod config;
pub(crate) mod display;

use chrono::{DateTime, FixedOffset, Utc};
use config::Config;
use rss::Channel;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

const READLIST_PATH: &str = "data/read_list.json";
pub(crate) type ReadList = HashMap<String, Vec<String>>;

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
struct Context {
    feeds: HashMap<String, Vec<String>>,
    config: Vec<Config>,
}

async fn feeds_and_config(configs: Vec<Config>) -> Result<Context, Box<dyn Error>> {
    let mut feeds_futures = HashMap::new();
    for config in configs.clone() {
        let feeds_future = new_feeds(config.feed.clone(), config.updated.clone());
        feeds_futures.insert(config.feed.clone(), feeds_future);
    }

    let current_time: DateTime<Utc> = Utc::now();
    let mut new_config = vec![];
    let mut feeds_to_read = HashMap::new();
    for (feed, future) in feeds_futures {
        let to_read = future.await?;
        feeds_to_read.insert(feed.clone(), to_read);
        new_config.push(Config {
            feed,
            updated: Some(current_time),
        });
    }

    Ok(Context {
        feeds: feeds_to_read,
        config: new_config,
    })
}

fn update_readlist(feeds: ReadList) -> Result<ReadList, Box<dyn Error>> {
    let read_list = fs::read_to_string(READLIST_PATH)?;
    let mut read_list: HashMap<String, Vec<String>> = serde_json::from_str(read_list.as_str())?;
    for (feed, mut to_read) in feeds.into_iter() {
        read_list.entry(feed).or_insert(vec![]).append(&mut to_read);
    }
    read_list.iter_mut().for_each(|(_, to_read)| {
        to_read.sort();
        to_read.dedup();
    });
    let data = serde_json::to_string_pretty(&read_list)?;
    fs::write(READLIST_PATH, data)?;
    Ok(read_list)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configs: Vec<Config> = config::feed_config()?;
    let conext = feeds_and_config(configs).await?;
    let read_list = update_readlist(conext.feeds)?;
    config::update(conext.config)?;
    display::display_feeds(read_list)?;

    Ok(())
}
