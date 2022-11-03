### Define and Instantiate Structs

A struct (*structure*) is a custom data type to package multiple related values with a meaningful name:
```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@example.com");
}
```
Note that the entire `user1` structure should be `mut`, Rust does not allow us to mark only certain fields as `mut`. 

Creating another instance with `user1` attributes that implement `Move` trait invalidates `user1` itself:
```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@example.com");
    
    let user2 = User {
        username: user1.username,
        email: user1.email,
        active: false,
        sign_in_count: 2,
    };
    println!("{}", user1.email); // THIS DOESN'T WORK: value borrowed here after move
    // move occurs because `user1.email` has type `String`, 
    // which does not implement the `Copy` trait
}
```
Using tuple structs with named fields:
```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let p = Point(100, 100, 100);
    println!("{}, {}, {}", p.0, p.1, p.2);
}
```

### Example project using `Struct` with `Debug` trait
`Debug` trait can be used to debug-print `struct`s (like `toString()` in Java):
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:?}", rec1);
}
```

`dbg!` macro can also be used to display `struct`s with `Debug` trait (but takes the ownership):
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rec1= dbg!(rec1);
}
```

### Methods

`impl` (implementation) block is used to extend `struct`s with methods. Each method should have `&self` as the first parameter (similar to Python `self`:
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 20,
        height: 50
    };
    println!("Rect1 can hold rect2: {}", rect1.can_hold(&rect2));
}
```

In Rust, associated functions can work without `self` as their first parameter, thus associated functions are not called *method*s (similar `companion object` in Kotlin). Associated functions are called using `Rectangle::square` syntax:
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side
        }
    }
}

fn main() {
    let square = Rectangle::square(30);
    println!("Square: {:?}", square);
}
```
