### `std::collections`

Rust's standard library includes [*`collections`*](https://doc.rust-lang.org/std/collections/index.html) module, unlike arrays or tuples, *`collections`* are stored on the heap, 
meaning that the amount of data does not have to be known at compile time. 

### Vectors
Creating a vector:
```
let vec: Vec<i32> = Vec::new();
let vec2 = vec![1,2,3]; // using macro
```
Updating a vector (must be `mut`):
```
let mut v = Vec::new(); // Rust infers the type later ;-)
v.push(4);
v.push(5); 
```
Reading by index from vector:
```
let v = vec![1,2,3,4,5];
let third = &v[2];
println!("{}", third);
```
Having both `mut` and immutable references does not compile:
```
let mut v = vec![1,2,3];
let first = &v[0]; //<- immutable borrow
v.push(4); //<- mutable borrow
println!("{}", first);
```
Iterating over a vector:
```
let v = vec![1,2,3];
for i in &v {
    println!("{}", i);
}

let mut v = vec![1,2,3];
for i in &mut v {
    *i += 50;
    println!("{}", i);
}
```
Vectors store elements of the same type.

### HashMaps
Hashmaps keep track of key-value pairs:
```
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Tottenham"), 44);
    map.insert(String::from("ManUtd"), 55);
    println!("{:?}", map);

    println!("{:?}", map.get(&String::from("ManUtd")).copied().unwrap_or(0));
    println!("{:?}", map.get(&String::from("Chelsea")).copied().unwrap_or(0));

    for (key, val) in &map {
        println!("{}: {}", key, val);
    }
}
```
Adding a value of if the key doesn't exist in the map:
```
    map.entry(String::from("Chelsea")).or_insert(50);
    println!("{:?}", map.get(&String::from("Chelsea")).copied().unwrap_or(0));
```
Updating a value based on the old value:
```
    let manutd = map.entry(String::from("ManUtd")).or_insert(0);
    *manutd += 3;
    println!("{:?}", map.get(&String::from("ManUtd")).copied().unwrap_or(0));
```
