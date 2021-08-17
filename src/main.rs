pub(crate) mod config;
pub(crate) mod display;
pub(crate) mod feeds;
pub(crate) mod readlist;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configs: Vec<config::Config> = config::feed_config()?;
    let conext = feeds::feeds_and_config(configs).await?;
    let read_list = readlist::update(conext.feeds)?;
    config::update(conext.config)?;
    display::display_feeds(read_list)?;

    Ok(())
}
