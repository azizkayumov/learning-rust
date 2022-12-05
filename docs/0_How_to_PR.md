Notes on how to PR in a Rust project:
1. Fix formatting issues:
```
cargo fmt --check
```
Or simply run `cargo fmt` to fix formatting issues at once.

2. Run Clippy lint (to catch common errors):
```
cargo clippy -- -D warnings -W clippy::pedantic
```
3. Avoid passing an argument of `&Vec<T>`, use `&[T]` instead (see [here](https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg)).
4. Remove unnecessary `#[allow(dead_code)]`s
