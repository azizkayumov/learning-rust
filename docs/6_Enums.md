### Enums
`enum` (enumerations) defines one of its possible variants.
```
enum IpAddressKind {
    V4,
    V6
}

struct IpAddress {
    kind: IpAddressKind,
    address: String
}

fn main() {
    let v4 = IpAddressKind::V4;
    let v6 = IpAddressKind::V6;
    
    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1")
    }
}
```

Instead, data can be attached directly to `enum`s:
```
enum IpAddress {
    V4(String),
    V6(String)
}

fn main() {
  let home = IpAddress::V4(String::from("127.0.0.1"));
}
```

Another advantage of `enum`s over `struct`s, an `enum` variants can hold different parameters:
```
enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String)
}

fn main() {
    let home = IpAddress::V4(127,0,0,1);
    let loopback = IpAddress::V6(String::from("::1"));
}
```
There is one similarity with `struct`s, `enum` can have its `impl` block:
```
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Message {
    fn call(&self) {
        // smth
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();
}
```

### The `Option` enum
Rust doesn't have a `null` feature, instead it has an enum called `Option` as part of the standard library to express the concept:
```
enum Option<T> {
    None, 
    Some(T),
}
```
`Option` does not have to be brought into scope, it is already in the prelude:
```
fn main() {
    let some_number = Some(32);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}
```
Then what's the point of having `Option` at all if we still need to check for `nullability`? Rust compiler forces programs to handle `None` cases:
```
let x: i8 = 5;
let y: Option<i8> = Some(7);
let res = x + y; /// THIS DOESN'T COMPILE: incompatible types i8 and Option<i8>!!!
```
To handle this case, [`unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) can be used (or any other function in its [docs](https://doc.rust-lang.org/std/option/enum.Option.html)):
```
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(7);
    let res = x + y.unwrap_or(7);
}
```

### `match` and `Option<T>`
`match` is perfectly used with `enum`s:
```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Handling `Option<T>` values with `match` is extremely easy:
```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let x = Some(220);
    let x = plus_one(x);
    println!("{:?}", x);
}
```
### `match`es are exhaustive

Rust doesn't compile `match` control flows without all cases covered:
```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
    }
} // THIS DOESN'T COMPILE: non-exhaustive patterns: `None` not covered
```
Rust compiler protects us from forgetting non-covered cases. Fortunately, the `_` placeholder can be used to cover all other cases:
```
enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

fn is_happy_day(day: Day) -> String {
    match day {
        Day::Fri | Day::Sat | Day::Sun => "a happy day".to_string(),
        _ => "is a sad day".to_string()
    }
}

fn main() {
    let friday = Day::Fri;
    println!("Today's {}", is_happy_day(friday));
}
```

### `if let` control flow
`if let` can be used to shorten `match`: 
```
enum Day { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

fn is_happy_day(day: Day) -> String {
    match day {
        Day::Fri | Day::Sat | Day::Sun => "a happy day".to_string(),
        _ => "is a sad day".to_string()
    }
}

fn main() {
    let chuseok: Option<Day> = Some(Day::Mon);
    if let chuseok = Some(Day::Mon) {
        println!("Monday is a happy day too");
    }
}
```
