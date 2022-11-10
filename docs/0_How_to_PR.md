Notes on how to PR in a Rust project:
1. Add a new line at the end of any file
2. Fix formatting issues:
```
cargo fmt --check
```
3. Run Clippy lint (to catch common errors):
```
cargo clippy -- -D warnings -W clippy::pedantic
```
