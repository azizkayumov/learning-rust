fn main() {
    println!("Hello, world!");
    // Rust strings use the least amount of bytes possible to represent its chars
    println!("Size of char = {}", std::mem::size_of::<char>());
    println!("Size of \"a\" = {}", "a".len());
    println!("Size of \"ß\" = {}", "ß".len());
    println!("Size of \"国\" = {}", "国".len());
    println!("Size of \"𓅱\" = {}", "𓅱".len());
    println!("Size of \"안녕\" = {}", "안녕".len());
}