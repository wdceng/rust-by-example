// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
static mut THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    unsafe { n > THRESHOLD }
}

fn main() {
    let n = 16;

    unsafe {
        // Access constant in the main thread
        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THRESHOLD);
        println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

        // Error! Cannot modify a `const`.
        // THRESHOLD = 5;
        // FIXME ^ Comment out this line

        THRESHOLD = 5;
        println!("New threshold is {}", THRESHOLD);
    }
}
