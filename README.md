# rss-update

fetched feed updated based on last run.

### Prerequisites
1. rust
2. git
3. cargo

### Usage
1. clone the github package.
2. cd in package dir
3. cargo run

### Adding feeds to track
1. Open data/config.json
2. Add feed link in format as others.
3. On first run it picks all feeds, if updated is not specified, else all feeds posted after updated
   date.

### Output Format
![output.png](images/output.png)

### Future Work
1. cli interface
    a. ability to track read files
    b. add feeds using cli instead of manually editing config
    c. ability to track read files from cli
