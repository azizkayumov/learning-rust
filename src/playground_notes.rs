fn number() -> i32 {
    return 8; // feel free to remove return and ;
}

fn multiply(a: i32, b: i32) -> i32 {
    let result = a * b;
    println!("{} * {} = {}", a, b, result);
    result
}

fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn print_country_by_ref(country_name: &String) {
    println!("{}", country_name);
}

fn add_another_country(country_name: &mut String) {
    println!("It was {}", country_name);
    country_name.push_str("+USA");
    println!("Now it is {}", country_name);
}

fn main() {
    // this is a comment!
    /*println!("Hello, world!1");
    println!("Hello, world!2");*/
    print!("Hello, World!");
    println!("Hello, world!");

    let number1 = 100; // Rust compiler chooses i32 by default
    println!("{}", number1 as u8 as char);

    let number2: u8 = 100;
    println!("{}", number2 as char);

    // Rust strings use the least amount of bytes possible to represent its chars
    println!("Size of char = {}", std::mem::size_of::<char>());
    println!("Size of \"a\" = {}", "a".len());
    println!("Size of \"ß\" = {}", "ß".len());
    println!("Size of \"国\" = {}", "国".len());
    println!("Size of \"𓅱\" = {}", "𓅱".len());
    println!("Size of \"안녕\" = {}", "안녕".len());

    let slice1 = "Hello";
    println!(
        "{} is {} chars and {} bytes long",
        slice1,
        slice1.chars().count(),
        slice1.len()
    );

    let slice = "안녕";
    println!(
        "{} is {} chars and {} bytes long",
        slice,
        slice.chars().count(),
        slice.len()
    );

    // Rust compiler can guess a type, but you need to tell it for 2 reasons:
    // 1. You're doing smth complex, and the compiler does not know what to do
    // 2. You want a different type
    let age: u8 = 72;
    // or
    let age2: u8 = 72_u8;
    println!("age = {}, age2 = {}", age, age2);

    // It doesn't matter how many _s you use:
    let beauty = 1_000_000______0111_i64;
    println!("{}", beauty);

    // Rust chooses f64 by default
    let float_number = 5.5;
    println!("{}", float_number);

    // two different types do not work together, you should cast one of them:
    let f32_number: f32 = 5.5;
    let fsum = float_number + f32_number;
    println!("{}", fsum);

    // function calls are the same
    println!("{}", number());

    let result = multiply(16, 16);
    println!("{}", result);

    // you can use a code block similar to a lambda function:
    let awesome_number = {
        let second_number = 8;
        second_number + 7
    };
    println!("{}", awesome_number);

    // DISPLAY "{}" and DEBUG{:?} prints
    // use DEBUG print when you don't know the type
    let really_nothing = ();
    println!("{:?}", really_nothing);

    // Smallest and largest numbers
    println!(
        "The smallest i8 is {} and the biggest i8 is {}",
        i8::MIN,
        i8::MAX
    );
    println!(
        "The smallest u8 is {} and the biggest u8 is {}",
        u8::MIN,
        u8::MAX
    );
    println!(
        "The smallest i16 is {} and the biggest i16 is {}",
        i16::MIN,
        i16::MAX
    );
    println!(
        "The smallest u16 is {} and the biggest u16 is {}",
        u16::MIN,
        u16::MAX
    );
    println!(
        "The smallest i32 is {} and the biggest i32 is {}",
        i32::MIN,
        i32::MAX
    );
    println!(
        "The smallest u32 is {} and the biggest u32 is {}",
        u32::MIN,
        u32::MAX
    );
    println!(
        "The smallest i64 is {} and the biggest i64 is {}",
        i64::MIN,
        i64::MAX
    );
    println!(
        "The smallest u64 is {} and the biggest u64 is {}",
        u64::MIN,
        u64::MAX
    );
    println!(
        "The smallest i128 is {} and the biggest i128 is {}",
        i128::MIN,
        i128::MAX
    );
    println!(
        "The smallest u128 is {} and the biggest u128 is {}",
        u128::MIN,
        u128::MAX
    );
    println!("{}", std::mem::size_of::<usize>());
    println!("{}", std::mem::size_of::<usize>());

    // when you declare a variable with let, it is immutable:
    let mut immutable_number = 32;
    immutable_number = 34;
    println!("immutable_number = {}", immutable_number);

    // shadowing existing variable names
    let my_number = 32;
    let my_number: f64 = 32.8;
    println!("the shadowed integer becomes a float: {}", my_number);

    // pointers are like in C++ pointers: reference -> value
    let my_variable = 8;
    let my_reference = &my_variable; // Read it: my_reference is a reference to my_variable
    println!("Reference: {}", my_reference); // references BORROW the value, but don't own it!

    // More printing
    println!(
        "You can write
over many lines"
    );

    // print bytes
    println!("{:?}", b"This will print the bytes");
    // put r# and b together:
    println!("{:?}", br##"This is a hashtag #"##);

    // Unicode escape:
    println!("{:X}", '행' as u32);
    println!("\u{D589}");

    // pointer address:
    let my_number = 8;
    let my_reference = &my_reference;
    println!("{:p}", my_reference);

    // or binary, hex and octal:
    let my_number = 8;
    println!(
        "Binary: {:b}, Hex: {:x}, Octal: {:o}",
        my_number, my_number, my_number
    );

    // change the order of printing
    println!("{2}, {1}, {0}", "a", "b", "c");

    // or add name for each printables:
    println!(
        "{age}, {name}, {phone}",
        name = "Aziz",
        age = 26,
        phone = 12312312
    );

    // Two types of strings: &str and String
    // &str is very fast, String is slower with more functions
    // You don't own &str, it is a reference
    let name = "서태지";
    let other_name = String::from("Adrian Fahrenheit Țepeș");
    println!("{:p}", name);
    println!("{}", other_name);

    // emojis are okay
    let name = "😂";
    println!("My name is {}", name);
    println!(
        "String is always {:?} bytes. It is sized!",
        std::mem::size_of::<String>()
    );
    println!(
        "Some other sized types: i8({}), u64({}), f64({}) etc.",
        std::mem::size_of::<i8>(),
        std::mem::size_of::<u64>(),
        std::mem::size_of::<f64>()
    );
    println!(
        "But str is not sized: 'aziz'({:?}), '서태지'({:?})",
        std::mem::size_of_val("aziz"),
        std::mem::size_of_val("서태지")
    );

    // many ways to make a string:
    let my_name = "Aziz";
    let my_country = "Uzbek";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}",
        my_name, my_country, my_home
    );
    println!("{}", together);
    let my_string = String::from("Hey Aziz");
    println!("{}", my_string);

    let try_to_make_it_a_string: String = "Try to make a String".into();
    println!("{}", try_to_make_it_a_string);

    // const does not change
    // static has a fixed memory location, can act as global var
    const NUMBER_OF_MONTHS: u32 = 12;
    static API_KEY: &str = "asdas";
    println!("{}", NUMBER_OF_MONTHS);

    // More on references
    // Rust uses references to make sure that all memory access is safe
    let country = String::from("Uzbekistan");
    let ref1 = &country;
    let ref2 = &country;
    println!("{}, {}", ref1, ref2);
    // but this is not okay:
    /*
    let another_ref = {
        let city = String::from("Tashkent"); /// <-- This guy does not LIVE ENOUGH!!
        let ref3 = &city;
        ref3
    };
    println!("{}", another_ref);
    */

    // Mutable references (just like in C++): use * to change the value
    // & is called "reference"
    // * is called "dereference"
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 95;
    println!("{}", my_number);
    // but this is not okay:
    /*
    let triple_ref = &&&mut my_number;
    ***triple_ref -= 95;
    println!("{}", my_number);
    */
    // A mutable reference can have only ONE level of reference: &mut my_number (only one editor)
    // An immutable reference can have many: &&&my_number (many listeners)
    let my_number = 8;
    let triple_ref = &&&my_number;
    println!("{}", ***triple_ref);
    // Shadowing doesn't destroy a value, it BLOCKS it.
    let country = String::from("Germany");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);

    // Give references to functions
    let country = String::from("Korea");
    print_country(country); // country value moved here, meaning that it is now dead for this context!
                            // print_country(country); // <-- This doesn't work: value used here after move
                            // So give references instead:
    let country = String::from("Japan");
    print_country_by_ref(&country);
    print_country_by_ref(&country); // This means: You can look at it, but I keep it!

    let mut country = String::from("Canada");
    add_another_country(&mut country);
    // Copy types: integers, floats, booleans and char
    // These types are all on the stack, so the compiler just copies them (the sizes are known -> easy to copy)
    // String is a clone type: e.g. it can be cloned to pass it to a function
    // but in most cases, just pass its reference (clone operation is heavy!)

    // Arrays: declared as [type; size]
    // 1. Array size can't be changed
    // 2. Array elements are always the same type
    let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    println!("{:?}", weekdays);

    let dogs = ["Dog"; 5]; // 5 "Dog"s
    println!("{:?}", dogs);
    println!("{}", dogs[1]);

    // Slicing the array (exclusive: e.g. the range '0..2' includes 0th and 1st elements).
    let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", &digits[1..4]); // from 1 until 4 (exclusive)
    println!("{:?}", &digits[..4]); // from start until 4
    println!("{:?}", &digits[4..]); // 4 until the end
    println!("{:?}", &digits[..]); // everything

    // Arrays are fast, but with less functionality
    // Vectors are slow, but with more functionality
    let mut employees = Vec::new();
    employees.push("Aziz"); // now Rust compiler knows the type -> no problem
    employees.push("Scott");
    println!("{:?}", employees);
    // Vector with a tuple
    let mut players: Vec<(&str, u8)> = Vec::new();
    players.push(("Ronaldo", 36));
    println!("{:?}", players);
    // Using vec!
    let weekends = vec!["Sat", "Sun"];
    println!("{:?}", weekends);
    // Slicing vectors (like in arrays)
    println!("{:?}", &players[..1]);
    // Vector has a capacity, when the number of elements reach its capacity
    // it doubles its capacity and reallocate everything (slow)
    let mut players = Vec::new();
    players.push("Ronaldo");
    println!("Capacity = {}", players.capacity());
    players.push("Messi");
    players.push("Mbappe");
    players.push("Neymar");
    println!("Capacity = {}", players.capacity());
    players.push("Rashford");
    println!("Capacity = {}", players.capacity());
    // but we can make it faster (by giving it a capacity):
    let mut players = Vec::with_capacity(8);
    players.push("Martinez");
    println!("Capacity = {}", players.capacity());
    // conversion of arrays to Vectors with into():
    let ages: Vec<u8> = [16, 18, 20, 22].into();
    println!("{:?}", ages);
    // create 2d 10x10 vector:
    let dp = vec![vec!(0; 10); 10];
    println!("{:?}", dp);

    // Tuples (similar to Python tuples)
    let random_tuple = ("Ronaldo", 37, 12_000_000);
    println!("{:?}", random_tuple);
    let (a, b, c) = (1, 2, 3);
    println!("{}, {}, {}", a, b, c);

    // Control flow
    // If statement: same with other languages with no brackets (&& ||)
    let my_number = 5;
    if my_number == 7 && my_number >= 7 {
        println!("This will never print!");
    } else if my_number == 6 || my_number == 8 {
        println!("This will too never print!");
    } else {
        println!("This guy prints!");
    }
    // Combine if+elses with match keyword:
    match my_number {
        7 => println!("This will never print!"),
        6 => println!("This too will never print!"),
        _ => println!("This guy prints!"),
    };
    // match tuples
    let country = "Korea";
    let population = "55mln";
    match (country, population) {
        ("USA", "320mln") => println!("Silicon valley"),
        ("Korea", "55mln") => println!("Gangnam"),
        _ => println!("This guy"),
    };
    // match has to return the same type
    let special_number = match my_number {
        7 => my_number * 14 + 27,
        _ => my_number,
    };

    println!("{}", special_number);
}
