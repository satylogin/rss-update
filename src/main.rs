pub(crate) mod config;
pub(crate) mod display;
pub(crate) mod feeds;
pub(crate) mod readlist;

use chrono::{DateTime, NaiveDate, Utc};
use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn base_dir() -> String {
    let base_path = Path::new(&dirs::home_dir().unwrap()).join(".rss-update-cli");
    String::from(base_path.to_str().unwrap())
}

// App level cli constants
const APP: &str = "rss-update";
const VERSION: &str = "0.1";
const ABOUT: &str = "To track and fetch updates on rss feeds.";

// Cli constants for action: generate pretty read list.
const UNREAD: &str = "unread";
const UNREAD_ABOUT: &str = "Display contents of read list on terminal.";

// Cli constants for action: add new source
const ADD: &str = "add";
const ADD_ABOUT: &str = "Add new feed source to track.";

// Cli constants for action: setup
const SETUP: &str = "setup";
const SETUP_ABOUT: &str = "Set up config for traking feeds.";

// Cli constants for action: tracking feeds
const TRACKING: &str = "tracking";
const TRACKING_ABOUT: &str =
    "Lists feeds that are currently being tracked along with its metadata.";

// Cli constants for action: remove
const REMOVE: &str = "remove";
const REMOVE_ABOUT: &str = "to remove feed from tracking";

// Cli constants for action: read
const READ: &str = "read";
const READ_ABOUT: &str = "to mark post as read.";

const USER_DATE_FORMAT: &str = "%Y-%m-%d";

fn parse_args() -> ArgMatches<'static> {
    App::new(APP)
        .version(VERSION)
        .about(ABOUT)
        .subcommand(App::new(UNREAD).about(UNREAD_ABOUT))
        .subcommand(
            App::new(ADD)
                .about(ADD_ABOUT)
                .arg(Arg::from_usage(
                    "--from [DATE] 'date to start tracking in YYYY-MM-DD (remember to pad with 0)'",
                ))
                .arg(Arg::from_usage("--feed [FEED] 'rss feed to track'").required(true)),
        )
        .subcommand(App::new(SETUP).about(SETUP_ABOUT))
        .subcommand(App::new(TRACKING).about(TRACKING_ABOUT))
        .subcommand(App::new(REMOVE).about(REMOVE_ABOUT).arg(
            Arg::from_usage("--feed [FEED] `rss feed to remove from tracking.`").required(true),
        ))
        .subcommand(
            App::new(READ)
                .about(READ_ABOUT)
                .arg(Arg::from_usage("--post [URL] `post url to mark as read.`").required(true)),
        )
        .get_matches()
}

fn unread() -> Result<(), Box<dyn Error>> {
    display::display_feeds(readlist::unread()?)
}

fn add_feed(args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    let tracking_date = args
        .value_of("from")
        .map(|d| {
            let d = NaiveDate::parse_from_str(d, USER_DATE_FORMAT).ok().unwrap();
            DateTime::from_utc(d.and_hms(0, 0, 0), Utc)
        })
        .unwrap_or(Utc::now());
    config::update(config::Config {
        feed: args.value_of("feed").unwrap().to_string(),
        updated: Some(tracking_date),
    })?;
    Ok(())
}

fn setup() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(base_dir())?;
    config::setup()?;
    readlist::setup()?;
    Ok(())
}

fn tracking() -> Result<(), Box<dyn Error>> {
    display::display_configs(config::get()?)
}

fn remove_feed(args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    let feed = args.value_of("feed").unwrap().to_string();
    config::remove(feed)?;
    Ok(())
}

fn mark_read(args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    let post = args.value_of("post").unwrap().to_string();
    readlist::mark_read(post)?;
    Ok(())
}

async fn fetch_new_feeds() -> Result<(), Box<dyn Error>> {
    let configs = config::get()?;
    let conext = feeds::feeds_and_config(configs).await?;
    let readlist = readlist::update(conext.feeds)?;
    config::replace(conext.configs)?;
    display::display_feeds(readlist)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();
    match args.subcommand() {
        (UNREAD, Some(_)) => unread(),
        (ADD, Some(s_args)) => add_feed(s_args),
        (SETUP, Some(_)) => setup(),
        (TRACKING, Some(_)) => tracking(),
        (REMOVE, Some(s_args)) => remove_feed(s_args),
        (READ, Some(s_args)) => mark_read(s_args),
        _ => fetch_new_feeds().await,
    }
}
