### Variables
By default, variables are immutable:
```
let x = 5;
```
To make them mutable, use `mut`:
```
let mut x = 5;
x = 6;
```
Use `const` to create constants (cannot be `mut`):
```
const WEEK_IN_HOURS: u32 = 7 * 24;
```
Shadowing a variable only effects its scope:
```
let x = 5;
let x = x + 1;

{
    let x = x * 2;
    println!("x = {x}"); // prints 12
}

println!("x = {x}"); // prints 6
```

### Data Types
Rust is a statically-typed language, the compiler must know the types at compile time:
```
let guess: u32 = "42".parse().expect("Not a number");
```
Scalar types: integers, floating-point numbers, Booleans and chars.  
Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
Unsigned ints:   `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
`isize` and `usize` depend on the architecture of your computer. 
Floats: `f32`, `f64`.
Boolean: `bool`
The character type: `char`

```
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2; // Results in 1.7608695652173911
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
    
    // characters
    let c = 'z';
    let cat = '😻';
}
```
A tuple is a grouping of multiple variables into one compound type:
```
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Arrays in Rust must have a fixed length and elements of the same type:
```
let a = [1,2,3,4,5];
```
Arrays are stored in stack rather than heap. 
Initialize an array to contain the same value for each element:
```
let a = [3; 5]; // [3,3,3,3,3]
```
