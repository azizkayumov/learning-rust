Rust allows this *module system* to organize Rust projects:
- **Package**s - a Cargo feature that build, test and share crates. 
- **Crate**s - a tree of modules that produces a library or executable
- **Module**s and `use` - to control the organization, scope and privacy of paths
- **Path**s - a way of naming an item such as `struct`, function or module.

### Packages and Crates
A *crate* is the smallest amount of code that Rust compiler considers at a time (in other words, it is basically a library).
Creates can come in two forms: **binary** and **library** crates. Binary crates have the `main` function whereas libraries don't have. 

A *package* is a bundle of one or more crates that provide a set of functionality. Cargo creates a package with `cargo new my_project`:
```
$ cargo new my-project
     Created binary (application) `my-project` package
```

### Modules
Modules let us organize Rust code for readability, easy reuse and privacy. A code within a module is private by default:
```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
This creates a module tree of:
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
### Paths in the Module tree
Using an absolute path or relative path should be decided for a project.     
`pub` is used to expose some public item in a module:
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
We can make some of the properties of `struct` private using `pub`:
```
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); THIS IS PRIVATE
}
```

Bring paths into scope with `use` to create a symbolic shortcut to the path:
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`as` can be used to provide a new local name to a path:
```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}

fn function2() -> IoResult<()> {}
```

### Using external packages
To use an external package, add it to `Cargo.toml`:
```
rand = "0.8.3"
```
and `use`:
```
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

### Using nested paths to clean up many `use`s:
```
use std::cmp::Ordering;
use std::io;
```
equals:
```
use std::{cmp::Ordering, io};
```

### The Glob operator `*`
Bring all public items into scope:
```
use std::collections::*;
```
