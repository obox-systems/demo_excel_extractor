# Extract by keywords

This program will read keywords in the `scrape.xlsx` file and extract all books from `https://pragprog.com/titles/` that contain those keywords back to the same sheet.

## Try it out!
1. Install [Rust](https://rustup.rs/)
2. Edit `scrape.xlsx` to add keywords you need and run:
```
$ cargo run
```
The program will update that file with the books it found.
