use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    // For example we can easily convert a str into a String
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{}", my_string);

    //The Into trait is simply the reciprocal of the From trait. It defines how to convert a type into another type.
    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
    println!("The value is {}", num.value);
}
