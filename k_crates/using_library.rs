// extern crate creating_a_library; // May be required for Rust 2015 edition or earlier

fn main() {
    creating_a_library::public_function();

    // Error! `private_function` is private
    //creating_a_library::private_function();

    creating_a_library::indirect_access();
}
