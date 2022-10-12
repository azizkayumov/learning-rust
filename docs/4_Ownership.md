The concept of ownership is the unique feature of Rust that guarentees memory safety without needing a garbage collector. 

### Stack vs Heap
Pushing to stack is fast and efficient, so it is used for fixed-sized values that can be known at compile time. Heap is used for dynamically allocated values with unknown values at compile time (e.g. `String`). 

### Ownership
The concept of ownership addresses:
- keeping track of what code is using what data on the heap
- minimizing duplicate data on the heap
- cleaning up unused data on the heap so you don't run out of memory
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
```
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }   // this scope is now over, and s is no longer valid, drop() is called
```
In some languages, unused variables are cleaned up by GC (Garbage Collector). Rust takes a different approach with `ownership`, once the variable that owns a chunk of memory on the heap goes out of scope, its memory is returned. Rust uses a special function called `drop` which is called automatically at the closing curly brackets. 

