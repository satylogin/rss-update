# rss-update
Supports both rss and atom feeds.

fetches feeds updated based on last run. This is still a work in progress and things are likely to
change.

### Prerequisites
* Cargo: You can get cargo by installing [rustup](https://www.rust-lang.org/learn/get-started) and 
this is as easy as running single command.

### Installation
1. cargo install rss-update
2. To verify installationn: `rss-update --help`
```bash
rss-update 0.1
To track and fetch updates on rss feeds.

USAGE:
    rss-update [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add         Add new feed source to track.
    help        Prints this message or the help of the given subcommand(s)
    read        to mark post as read.
    remove      to remove feed from tracking
    setup       Set up config for traking feeds.
    tracking    Lists feeds that are currently being tracked along with its metadata.
    unread      Display contents of read list on terminal.
```
3. To setup: `rss-update setup`.

### Usage
* To add new feeds: `rss-update add --feed <FEED>`
* To fetch new posts in feed: `rss-update`
* To show unread posts: `rss-update unread`
* To mark posts as read: `rss-feed read --post <URL>`
* To list tracking feeds: `rss-feed tracking`
* To remove feed from tracking: `rss-feed remove --feed <FEED>`


* For performing specific subcommand: `rss-update subcommand --opts...`

`rss-update --help` should return output as shown above. you can get spefic command help by running
help on subcommand like 

`rss-update add --help`.
```bash
rss-update-add 
Add new feed source to track.

USAGE:
    rss-update add [OPTIONS] --feed <FEED>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --feed <FEED>    rss feed to track
        --from <DATE>    date to start tracking in YYYY-MM-DD (remember to pad with 0)
```

### Output Format
![output.png](https://raw.githubusercontent.com/satylogin/rss-update/main/images/output.png)
