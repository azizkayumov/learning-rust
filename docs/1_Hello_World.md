Create a Rust file `hello.rs` and paste the following:
```
fn main() {
    println!("Hello, World!");
}
```
Compile and run the Rust file:
```
> rustc main.rs
> ./main
```

`Cargo` is the build system of Rust to handle dependency management, building and testing tasks. 
Create a project with `Cargo`:
```
cargo new hello_cargo
cd hello_cargo
```
Building the project (generates binary files):
```
cargo build
```
Build & run the project:
```
cargo run
```
Quickly check the project for compilation:
```
cargo check
```
Building for release:
```
cargo build --release
```
