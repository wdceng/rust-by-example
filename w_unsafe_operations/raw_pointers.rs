fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
        println!("Unsafe 1: {}", *raw_p);
    }
    println!("Safe 2: {:?}", raw_p);
    println!("Safe 3: {:?}", &raw_p);
}
