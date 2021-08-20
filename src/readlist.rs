use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn readlist_path() -> String {
    let readlist_path = Path::new(&crate::base_dir()).join("read_list.json");
    String::from(readlist_path.to_str().unwrap())
}

pub(crate) type ReadList = HashMap<String, Vec<String>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn _update(feeds: ReadList, mut readlist: ReadList) -> Result<ReadList> {
    for (feed, mut to_read) in feeds.into_iter() {
        readlist.entry(feed).or_insert(vec![]).append(&mut to_read);
    }
    readlist.iter_mut().for_each(|(_, to_read)| {
        to_read.sort();
        to_read.dedup();
    });
    Ok(readlist)
}

pub(crate) fn update(feeds: ReadList) -> Result<ReadList> {
    let read_list = fs::read_to_string(readlist_path())?;
    let read_list: ReadList = serde_json::from_str(read_list.as_str())?;
    let read_list = _update(feeds, read_list)?;
    let data = serde_json::to_string_pretty(&read_list)?;
    fs::write(readlist_path(), data)?;
    Ok(read_list)
}

pub(crate) fn replace(readlist: ReadList) -> Result<ReadList> {
    let data = serde_json::to_string_pretty(&readlist)?;
    fs::write(readlist_path(), data)?;
    Ok(readlist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readlist_path() {
        assert!(super::readlist_path().starts_with("/"))
    }

    #[test]
    fn test_update() -> Result<()> {
        let mut read_list = ReadList::new();
        read_list.insert(
            "feed1".to_string(),
            vec!["post1".to_string(), "post2".to_string()],
        );
        read_list.insert(
            "feed2".to_string(),
            vec!["post3".to_string(), "post4".to_string()],
        );
        let mut feeds = ReadList::new();
        feeds.insert(
            "feed1".to_string(),
            vec!["post1".to_string(), "post3".to_string()],
        );
        feeds.insert(
            "feed3".to_string(),
            vec!["post5".to_string(), "post6".to_string()],
        );
        let output = _update(feeds, read_list)?;

        assert_eq!(3, output.len());
        assert_eq!(
            vec![
                "post1".to_string(),
                "post2".to_string(),
                "post3".to_string()
            ],
            output["feed1"]
        );
        assert_eq!(
            vec!["post3".to_string(), "post4".to_string()],
            output["feed2"]
        );
        assert_eq!(
            vec!["post5".to_string(), "post6".to_string()],
            output["feed3"]
        );
        Ok(())
    }
}
