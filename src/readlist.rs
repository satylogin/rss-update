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

fn _update(feeds: ReadList, path: String) -> Result<ReadList> {
    let read_list = fs::read_to_string(&path)?;
    let mut read_list: ReadList = serde_json::from_str(read_list.as_str())?;
    for (feed, mut to_read) in feeds.into_iter() {
        read_list.entry(feed).or_insert(vec![]).append(&mut to_read);
    }
    read_list.iter_mut().for_each(|(_, to_read)| {
        to_read.sort();
        to_read.dedup();
    });
    let data = serde_json::to_string_pretty(&read_list)?;
    fs::write(&path, data)?;
    Ok(read_list)
}

fn _replace(read_list: ReadList, path: String) -> Result<ReadList> {
    let data = serde_json::to_string_pretty(&read_list)?;
    fs::write(&path, data)?;
    Ok(read_list)
}

pub(crate) fn update(feeds: ReadList) -> Result<ReadList> {
    _update(feeds, readlist_path())
}

pub(crate) fn replace(read_list: ReadList) -> Result<ReadList> {
    _replace(read_list, readlist_path())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn check_readlist_path_is_absolute() {
        assert!(super::readlist_path().starts_with("/"))
    }

    #[test]
    fn test_update() -> Result<()> {
        let file = NamedTempFile::new()?;
        let mut read_list = ReadList::new();
        read_list.insert(
            "feed1".to_string(),
            vec!["post1".to_string(), "post2".to_string()],
        );
        read_list.insert(
            "feed2".to_string(),
            vec!["post3".to_string(), "post4".to_string()],
        );
        let data = serde_json::to_string(&read_list)?;
        let path = String::from(file.path().to_str().unwrap());
        fs::write(&path, data)?;

        let mut feeds = ReadList::new();
        feeds.insert(
            "feed1".to_string(),
            vec!["post1".to_string(), "post3".to_string()],
        );
        feeds.insert(
            "feed3".to_string(),
            vec!["post5".to_string(), "post6".to_string()],
        );

        let output = _update(feeds, path)?;
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

    #[test]
    fn test_replace() -> Result<()> {
        let file = NamedTempFile::new()?;
        let mut read_list = ReadList::new();
        read_list.insert(
            "feed".to_string(),
            vec!["post1".to_string(), "post2".to_string()],
        );
        let data = _replace(
            read_list.clone(),
            String::from(file.path().to_str().unwrap()),
        )?;
        assert_eq!(data, read_list);
        Ok(())
    }
}
