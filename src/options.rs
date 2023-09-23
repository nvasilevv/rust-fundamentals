// Rust does not implement a null data type!
// Option in rust works in place of null data type.
// Essentially, `option` is an enum which returns `Some` if the value exists and `None` if the value does not


fn main() {
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(15);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    println!("{}", value)
}