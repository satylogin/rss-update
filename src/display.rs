use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub(crate) fn display_feeds(feeds: HashMap<String, Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for (feed, to_read) in feeds {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
        write!(&mut stdout, "feed: {}, ", feed)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
        writeln!(&mut stdout, "total unread: {}", to_read.len())?;
        for tr in to_read {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            writeln!(&mut stdout, "  {}", tr)?;
        }
    }
    Ok(())
}
