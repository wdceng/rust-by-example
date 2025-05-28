// #[outer_attribute] applies to the item immediately following it. Some examples of items are: a function, a module declaration, a constant, a structure, an enum. Here is an example where attribute #[derive(Debug)] applies to the struct Rectangle:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {}
