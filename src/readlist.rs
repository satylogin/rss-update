use std::collections::HashMap;
use std::error::Error;
use std::fs;

const READLIST_PATH: &str = "data/read_list.json";
pub(crate) type ReadList = HashMap<String, Vec<String>>;

pub(crate) fn update(feeds: ReadList) -> Result<ReadList, Box<dyn Error>> {
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
