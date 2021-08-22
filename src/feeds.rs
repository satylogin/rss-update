use crate::config::ConfigList;
use crate::readlist::ReadList;
use chrono::{DateTime, Utc};
use futures::future;
use std::error::Error;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub(crate) type Posts = Vec<String>;

async fn new_posts_from_feed(
    feed: atom_syndication::Feed,
    updated: Option<DateTime<Utc>>,
    peek_time: DateTime<Utc>,
) -> Result<Posts> {
    let updated = updated.unwrap_or(Utc::now());
    let links = feed
        .entries()
        .into_iter()
        .filter(|entry| {
            let date_time = DateTime::parse_from_rfc3339(entry.updated()).ok().unwrap();
            date_time > updated && date_time <= peek_time
        })
        .map(|entry| String::from(entry.links()[0].href()))
        .collect::<Vec<_>>();
    Ok(links)
}

async fn new_posts_from_channel(
    channel: rss::Channel,
    updated: Option<DateTime<Utc>>,
    peek_time: DateTime<Utc>,
) -> Result<Posts> {
    let updated = updated.unwrap_or(Utc::now());
    let links = channel
        .items()
        .into_iter()
        .filter(|item| {
            let parsed_date = DateTime::parse_from_rfc2822(item.pub_date().unwrap())
                .ok()
                .unwrap();
            parsed_date > updated && parsed_date <= peek_time
        })
        .map(|item| String::from(item.link().unwrap()))
        .collect::<Vec<_>>();
    Ok(links)
}

async fn new_posts(
    url: String,
    updated: Option<DateTime<Utc>>,
    peek_time: DateTime<Utc>,
) -> Result<Posts> {
    let data = reqwest::get(url.as_str()).await?.text().await?;
    let new_posts: Vec<String> = match data.parse::<syndication::Feed>()? {
        syndication::Feed::Atom(feed) => new_posts_from_feed(feed, updated, peek_time).await?,
        syndication::Feed::RSS(channel) => {
            new_posts_from_channel(channel, updated, peek_time).await?
        }
    };
    Ok(new_posts)
}

#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) feeds: ReadList,
    pub(crate) configs: ConfigList,
}

pub(crate) async fn feeds_and_config(
    mut configs: ConfigList,
    peek_time: DateTime<Utc>,
) -> Result<Context> {
    let feeds_futures = configs
        .iter()
        .map(|c| new_posts(c.feed.clone(), c.updated.clone(), peek_time.clone()))
        .collect::<Vec<_>>();
    let new_posts = future::try_join_all(feeds_futures).await?;

    let mut feeds = ReadList::new();
    for (config, to_read) in configs.iter_mut().zip(new_posts.into_iter()) {
        feeds.insert(config.feed.clone(), to_read);
        config.updated = Some(peek_time);
    }
    Ok(Context { feeds, configs })
}
