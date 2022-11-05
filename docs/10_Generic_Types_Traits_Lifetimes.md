In Rust:
- **Generics** - abstract replacement of concrete types (used in function and struct signatures).
- **Traits** define the behavior of generic types (to constraint generic types).
- **Lifetimes** are a variety of generics that that define the lifetime of references and how they relate to each other.

### Generic Data Types
In function definitions:
```
fn largest<T>(list: &[T]) -> &T {}
```
In struct definitions:
```
struct Point<T>{
    x: T,
    y: T
}
```
In enum definitions:
```
enum Option<T>{
    Some(T),
    None
}
```

### Traits
`trait` defines shared behavior of a particular type with other types. Traits can be used to constraint generic types. 
```

pub trait Work {
    fn work(&self);
}

pub struct CEO;
pub struct HR;
pub struct Developer;

impl Work for CEO{
    fn work(&self) {
        println!("Sleeping in the office..");
    }
}

impl Work for HR {
    fn work(&self) {
        println!("Head-hunting..!");
    }
}

impl Work for Developer {
    fn work(&self) {
        println!("Coding..");
    }
}
```
Traits can be passed as parameters:
```
pub fn make_activity(item: &impl Work){
    item.work();
}
```
or:
```
pub fn make_activity<T: Work>(item: &T){
    item.work()
}
```
Specify multiple *trait bounds* with `+`:
```
pub fn make_activity<T: Work + Hobby>(item: &T){
    ..
}
```
or even clearer:
```
pub fn make_activity<T>(item: &T)
where T: Work + Hobby
{
    ...
}
```
return type that implements `trait`:
```
pub fn activity() -> impl Work
```
