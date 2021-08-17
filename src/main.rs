pub(crate) mod config;
pub(crate) mod display;
pub(crate) mod feeds;
pub(crate) mod readlist;

use clap::{App, ArgMatches};
use std::error::Error;

// App level cli constants
const APP: &str = "rss-update";
const VERSION: &str = "0.1";
const ABOUT: &str = "to track and fetch updates on rss feeds";

// Cli constants for action: generate pretty read list.
const UNREAD: &str = "unread";
const UNREAD_ABOUT: &str = "display contents of read list on terminal";

fn parse_args() -> ArgMatches<'static> {
    App::new(APP)
        .version(VERSION)
        .about(ABOUT)
        .subcommand(App::new(UNREAD).about(UNREAD_ABOUT))
        .get_matches()
}

fn unread() -> Result<(), Box<dyn Error>> {
    let unread_feeds = readlist::update(readlist::ReadList::new())?;
    display::display_feeds(unread_feeds)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();
    if let Some(_) = args.subcommand_matches(UNREAD) {
        unread()?;
    } else {
        let configs: Vec<config::Config> = config::feed_config()?;
        let conext = feeds::feeds_and_config(configs).await?;
        let read_list = readlist::update(conext.feeds)?;
        config::update(conext.config)?;
        display::display_feeds(read_list)?;
    }

    Ok(())
}
