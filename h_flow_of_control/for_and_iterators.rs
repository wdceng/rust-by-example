fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    // iter() needs to burrow element from names
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // into_iter() consumes names
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Cannot be printed, names were consumed
    // println!("names: {:?}", names);
}
