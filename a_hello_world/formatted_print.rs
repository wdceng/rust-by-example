fn main() {
    // In general "{}" shall be automatically  replaced with argument "31"
    println!("{} days", 31);

    // Positional arguments
    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");

    // Named arguments
    println!(
        "{subject} {verb} for {object}",
        subject = "Dog",
        object = "ball",
        verb = "is running"
    );

    // Different formatting
    println!("Base 10: {}", 7);
    println!("Base 10: {:04}", 7);
    println!("Base 2: {:b}", 7);
    println!("Base 2: {:04b}", 7);
    println!("Base 8: {:o}", 7);
    println!("Base 8: {:04o}", 7);
    println!("Base 16: {:x}", 7);
    println!("Base 16: {:04x}", 7);

    // Right-justified text with specific width
    let number = 1;
    println!("{number:>4}");
    println!("{number:<4}");

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 2);
    println!("{number:->5}", number = 3);
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 4);
    println!("{number:-<5}", number = 5);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:>width$}", number = 1, width = 4);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // Disable "dead_code" which warn against unused module
    #[derive(Debug)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement fmt::Display.
    println!("This struct `{:?}` won't print...", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from a surrounding variable.
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:0>width$}");

    // Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
    let pi: f64 = 3.141592;
    println!("{pi:.3}");
}
