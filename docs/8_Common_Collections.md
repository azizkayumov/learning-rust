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
