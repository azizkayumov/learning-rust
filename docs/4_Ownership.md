The concept of ownership is the unique feature of Rust that guarentees memory safety without needing a garbage collector. 

### Stack vs Heap
Pushing to stack is fast and efficient, all data stored on the stack must have a fixed size that can be known at compile time. Heap is used for dynamically allocated values with an unknown size at compile time (e.g. `String`). 

### Ownership
The concept of ownership addresses:
- keeping track of what code is using what data on the heap
- minimizing duplicate data on the heap
- cleaning up unused data on the heap so you don't run out of memory.      

In other words, the main purpose of ownership is to manage heap data.

### Ownership rules
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped. 

### Variable scope
```
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }   // s is no longer valid here
```

### `&str` and `String`
`&str` is a fixed size text (immutable), so it can be bundled with the executable binary file at compile time. However, `String` is dynamically allocated (mutable) so it is stored in the heap. 

### Allocation and `drop` 
```
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }   // this scope is now over, and s is no longer valid, drop() is called
```
In some languages, unused variables are cleaned up by GC (Garbage Collector). Rust takes a different approach with `ownership`, once the variable that owns a chunk of memory on the heap goes out of scope, its memory is returned. Rust uses a special function called `drop` which is called automatically at the closing curly brackets. 

### Move
For fixed sized values (`i32`, `f64`), Rust performs `copy` of the value:
```
let a = 25;
let b = a;   // the value of a is copied.
```
But with `String`s, Rust performs `copy` of the pointer on the stack (also called `shallow copy`):
```
let s1 = String::from("Hello");
let s2 = s1;
```
This operation is called `move` because Rust moves `s1` to `s2` and invalidates `s1`. So this doesn't compile:
```
let s1 = String::from("Hello");
let s2 = s1; // s2 is the owner now
println!("{s1}"); // THIS DOESN'T WORK: value borrowed here after move!!!
```
Rust considers `s1` as invalid after move, so it only drops `s2` after `s2` goes out of scope.

### Deep copy
This copies the heap data instead of the pointer on the stack (also called `deep copy`)
```
let s1 = String::from("Hello");
let s2 = s1.clone();
```
Rust performs `deep copy` on these fixed sized types:
- All integers types such as `u64`
- All floating numbers: such as `f32`
- Boolean type: `bool`
- The character type: `char`
- Tuples with fixed-sized values such as `(i32, i32)`

These `copy` types implements a Rust trait called `Copy`.

### Ownership and Functions
When a dynamically-sized type is passed to a function as a parameter, the ownership also moves to the function:
```
fn takes_ownership(s: String){  // new owner of s
    println!("{s}"); // do some stuff with s
} // s is dropped

fn main() {
    let s = String::from("Hello!");
    takes_ownership(s);
    println!("{s}"); // THIS DOESN'T WORK: value borrowed here after move!!!
}
```
However, Rust copies the value for `copy` types, so the value is not moved:
```
fn takes_ownership(s: String){  // new owner of s
    println!("{s}"); // do some stuff with s
} // s is dropped

fn makes_copy(x: i32){  // new variable x
    println!("{x}"); // do stuff with x
}  // x is dropped

fn main() {
    let s = String::from("Hello!");
    takes_ownership(s);

    let a = 20;
    takes_copy(a);
    println!("{a}"); // this works
}
```

### Ownership and Return Values
When a variable with data on the heap goes out of scope, the value will be cleaned by `drop` unless the ownership of its data has been moved to another variable:
```
fn takes_ownership(s: String) -> String {  // new owner of s
    println!("{s}"); // do some stuff
    s // return the value to a new owner
}

fn main() {
    let s = String::from("Hello!");
    let s2 = takes_ownership(s);
    println!("{s2}");
}
```
