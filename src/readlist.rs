use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) type ReadList = HashMap<String, Vec<String>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn readlist_path() -> String {
    let readlist_path = Path::new(&crate::base_dir()).join("read_list.json");
    String::from(readlist_path.to_str().unwrap())
}

pub(crate) fn setup() -> Result<()> {
    let readlist_path = readlist_path();
    if Path::new(&readlist_path).is_file() {
        println!("readlist file already exists.");
    } else {
        println!("creating readlist path.");
        fs::write(readlist_path, "{}")?;
    }
    Ok(())
}

fn get() -> Result<ReadList> {
    let read_list = fs::read_to_string(readlist_path())?;
    Ok(serde_json::from_str(read_list.as_str())?)
}

pub(crate) fn unread() -> Result<ReadList> {
    Ok(_unread(get()?))
}

fn _unread(readlist: ReadList) -> ReadList {
    readlist
        .into_iter()
        .filter(|(_, readlist)| !readlist.is_empty())
        .collect()
}

fn _mark_read(mut readlist: ReadList, post: &str) -> ReadList {
    for to_read in readlist.values_mut() {
        *to_read = to_read
            .iter()
            .filter(|p| **p != post)
            .map(std::clone::Clone::clone)
            .collect::<Vec<_>>();
    }
    readlist
}

pub(crate) fn mark_read(post: &str) -> Result<ReadList> {
    let readlist = _mark_read(get()?, post);
    replace(readlist)
}

pub(crate) fn update(feeds: ReadList) -> Result<ReadList> {
    let read_list = get()?;
    let read_list = _update(feeds, read_list);
    let data = serde_json::to_string_pretty(&read_list)?;
    fs::write(readlist_path(), data)?;
    Ok(read_list)
}

fn _update(feeds: ReadList, mut readlist: ReadList) -> ReadList {
    for (feed, mut to_read) in feeds {
        readlist
            .entry(feed)
            .or_insert_with(Vec::new)
            .append(&mut to_read);
    }
    for to_read in readlist.values_mut() {
        to_read.sort();
        to_read.dedup();
    }
    readlist
}

pub(crate) fn replace(readlist: ReadList) -> Result<ReadList> {
    let data = serde_json::to_string_pretty(&readlist)?;
    fs::write(readlist_path(), data)?;
    Ok(readlist)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn readlist_from(tuples: Vec<(&str, Vec<&str>)>) -> ReadList {
        tuples
            .into_iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    v.into_iter()
                        .map(std::string::ToString::to_string)
                        .collect(),
                )
            })
            .collect()
    }

    #[test]
    fn test_readlist_path() {
        assert!(super::readlist_path().starts_with('/'));
    }

    #[test]
    fn test_update() {
        let readlist = readlist_from(vec![
            ("feed1", vec!["post1", "post2"]),
            ("feed2", vec!["post3", "post4"]),
        ]);
        let feeds = readlist_from(vec![
            ("feed1", vec!["post1", "post3"]),
            ("feed3", vec!["post5", "post6"]),
        ]);

        let output = _update(feeds.clone(), readlist.clone());
        assert_eq!(3, output.len());
        assert_eq!(
            vec![
                "post1".to_string(),
                "post2".to_string(),
                "post3".to_string()
            ],
            output["feed1"]
        );
        assert_eq!(readlist["feed2"], output["feed2"]);
        assert_eq!(feeds["feed3"], output["feed3"]);
    }

    #[test]
    fn test_unread() {
        let readlist = readlist_from(vec![
            ("feed1", vec!["post1", "post2"]),
            ("feed2", vec![]),
            ("feed3", vec!["post5", "post6"]),
        ]);
        let unread = _unread(readlist);
        assert!(unread.contains_key("feed1"));
        assert!(!unread.contains_key("feed2"));
        assert!(unread.contains_key("feed3"));
    }

    #[test]
    fn test_mark_read_existing_post() {
        let readlist = readlist_from(vec![
            ("feed1", vec!["post1", "post2"]),
            ("feed2", vec!["post3", "post4", "post5"]),
        ]);

        let output = _mark_read(readlist, "post4");

        let expected = readlist_from(vec![
            ("feed1", vec!["post1", "post2"]),
            ("feed2", vec!["post3", "post5"]),
        ]);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_mark_read_non_existing_post() {
        let readlist = readlist_from(vec![
            ("feed1", vec!["post1", "post2"]),
            ("feed2", vec!["post3", "post5"]),
        ]);
        let output = _mark_read(readlist.clone(), "post4");
        assert_eq!(readlist, output);
    }
}
