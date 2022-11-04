Notes on how to PR in a Rust project:
1. Add a new line at the end of any file
2. Run and fix warnings: 
```
cargo clippy -- -D warnings -W clippy::pedantic
```
