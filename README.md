# rss-update

Right now just iterates over feeds and get the link:

Output Format:
```rust
[src/main.rs:32] feeds_to_read = {
    "https://dev.to/feed/satylogin": [
        "https://dev.to/satylogin/indirect-coupling-story-bpo",
        "https://dev.to/satylogin/starship-because-it-s-too-damn-cool-3nfc",
        "https://dev.to/satylogin/playing-the-generator-game-2bk4",
        "https://dev.to/satylogin/how-i-misunderstood-lombok-1k44",
    ],
    "https://satylogin.medium.com/feed": [
        "https://satylogin.medium.com/debugging-tps-e2a3f2b8bdd2?source=rss-51c268decb24------2",
        "https://satylogin.medium.com/starship-because-its-too-damn-cool-503ffa5562fa?source=rss-51c268decb24------2",
        "https://medium.com/analytics-vidhya/playing-the-generator-game-8a3c22987722?source=rss-51c268decb24------2",
        "https://satylogin.medium.com/how-i-misunderstood-lombok-3cdfbed37f73?source=rss-51c268decb24------2",
        "https://satylogin.medium.com/indirect-coupling-story-1a374d5ef8d6?source=rss-51c268decb24------2",
        "https://medium.com/analytics-vidhya/tensorflow-batch-inference-using-sagemaker-spark-sdk-6ccb01f2e29c?source=rss-51c268decb24------2",
    ],
}
```
