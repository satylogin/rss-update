use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn readlist_path() -> String {
    let readlist_path = Path::new(&crate::base_dir()).join("read_list.json");
    String::from(readlist_path.to_str().unwrap())
}

pub(crate) type ReadList = HashMap<String, Vec<String>>;

pub(crate) fn update(feeds: ReadList) -> Result<ReadList, Box<dyn Error>> {
    let read_list = fs::read_to_string(readlist_path())?;
    let mut read_list: HashMap<String, Vec<String>> = serde_json::from_str(read_list.as_str())?;
    for (feed, mut to_read) in feeds.into_iter() {
        read_list.entry(feed).or_insert(vec![]).append(&mut to_read);
    }
    read_list.iter_mut().for_each(|(_, to_read)| {
        to_read.sort();
        to_read.dedup();
    });
    let data = serde_json::to_string_pretty(&read_list)?;
    fs::write(readlist_path(), data)?;
    Ok(read_list)
}
