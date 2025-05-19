# web-scraping using Rust Lambda runtime
1. Test locally by running the emulator:
```
cargo lambda watch
```

2. Invoke in another terminal:
```
cargo lambda --invoke --data-ascii "{ \"hello\": \"world\" }" web-scraper
```
