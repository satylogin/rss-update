use crate::config::Config;
use crate::readlist::ReadList;
use chrono::{DateTime, Utc};
use futures::future;
use std::error::Error;

async fn new_posts_from_feed(
    feed: atom_syndication::Feed,
    updated: Option<DateTime<Utc>>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let updated = updated.unwrap_or(Utc::now());
    let links = feed
        .entries()
        .into_iter()
        .filter(|entry| {
            let date_time = DateTime::parse_from_rfc3339(entry.updated()).ok().unwrap();
            date_time > updated
        })
        .map(|entry| String::from(entry.links()[0].href()))
        .collect::<Vec<_>>();
    Ok(links)
}

async fn new_posts_from_channel(
    channel: rss::Channel,
    updated: Option<DateTime<Utc>>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let updated = updated.unwrap_or(Utc::now());
    let links = channel
        .items()
        .into_iter()
        .filter(|item| {
            let parsed_date = DateTime::parse_from_rfc2822(item.pub_date().unwrap())
                .ok()
                .unwrap();
            parsed_date > updated
        })
        .map(|item| String::from(item.link().unwrap()))
        .collect::<Vec<_>>();
    Ok(links)
}

async fn new_posts(
    url: String,
    updated: Option<DateTime<Utc>>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let data = reqwest::get(url.as_str()).await?.text().await?;
    let new_posts: Vec<String> = match data.parse::<syndication::Feed>()? {
        syndication::Feed::Atom(feed) => new_posts_from_feed(feed, updated).await?,
        syndication::Feed::RSS(channel) => new_posts_from_channel(channel, updated).await?,
    };
    Ok(new_posts)
}

#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) feeds: ReadList,
    pub(crate) configs: Vec<Config>,
}

pub(crate) async fn feeds_and_config(configs: Vec<Config>) -> Result<Context, Box<dyn Error>> {
    let mut feeds_futures = vec![];
    for config in configs.iter() {
        feeds_futures.push(new_posts(config.feed.clone(), config.updated.clone()));
    }
    let new_feeds = future::join_all(feeds_futures).await;

    let current_time: DateTime<Utc> = Utc::now();
    let mut new_configs = vec![];
    let mut feeds_to_read = ReadList::new();
    for (config, to_read) in configs.into_iter().zip(new_feeds.into_iter()) {
        feeds_to_read.insert(config.feed.clone(), to_read?);
        new_configs.push(Config {
            updated: Some(current_time),
            ..config
        })
    }

    Ok(Context {
        feeds: feeds_to_read,
        configs: new_configs,
    })
}
