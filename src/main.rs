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
const ABOUT: &str = "to track and fetch updates on rss feeds.";

// Cli constants for action: generate pretty read list.
const UNREAD: &str = "unread";
const UNREAD_ABOUT: &str = "display contents of read list on terminal.";

// Cli constants for action: add new source
const ADD: &str = "add";
const ADD_ABOUT: &str = "add new feed source to track.";

// Cli constants for action: setup
const SETUP: &str = "setup";
const SETUP_ABOUT: &str = "set up config for traking feeds.";

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
        .get_matches()
}

fn unread() -> Result<(), Box<dyn Error>> {
    let unread_feeds: readlist::ReadList = readlist::update(readlist::ReadList::new())?
        .into_iter()
        .filter(|(_, read_list)| !read_list.is_empty())
        .collect();
    display::display_feeds(unread_feeds)?;
    Ok(())
}

fn add_feed(args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    let feed = args
        .value_of("feed")
        .expect("feed is required arg")
        .to_string();
    let tracking_date = args
        .value_of("from")
        .map(|d| {
            let d = NaiveDate::parse_from_str(d, USER_DATE_FORMAT).ok().unwrap();
            DateTime::from_utc(d.and_hms(0, 0, 0), Utc)
        })
        .unwrap_or(Utc::now());
    let new_config = config::Config {
        feed,
        updated: Some(tracking_date),
    };
    let mut configs: Vec<config::Config> = config::feed_config()?;
    for c in configs.iter() {
        if c.feed == new_config.feed {
            println!("found duplicate config: {:?}, skipping update.", &c);
            return Ok(());
        }
    }
    println!("adding config for: {:?}", &new_config);
    configs.push(new_config);
    config::update(configs)?;
    Ok(())
}

fn setup() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(base_dir())?;
    let config_path = config::config_path();
    if Path::new(&config_path).is_file() {
        println!("config file already exists.");
    } else {
        println!("creating config path.");
        fs::write(config_path, "[]")?;
    }
    let readlist_path = readlist::readlist_path();
    if Path::new(&readlist_path).is_file() {
        println!("readlist file already exists.");
    } else {
        println!("creating readlist path.");
        fs::write(readlist_path, "{}")?;
    }
    Ok(())
}

async fn fetch_new_feeds() -> Result<(), Box<dyn Error>> {
    let configs: Vec<config::Config> = config::feed_config()?;
    let conext = feeds::feeds_and_config(configs).await?;
    let read_list = readlist::update(conext.feeds)?;
    config::update(conext.config)?;
    display::display_feeds(read_list)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();
    if let Some(_) = args.subcommand_matches(UNREAD) {
        unread()?;
    } else if let Some(s_args) = args.subcommand_matches(ADD) {
        add_feed(s_args)?;
    } else if let Some(_) = args.subcommand_matches(SETUP) {
        setup()?;
    } else {
        fetch_new_feeds().await?;
    }

    Ok(())
}
