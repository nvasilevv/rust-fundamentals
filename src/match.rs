// The match statement in rust is comperable to the `switch` statement in other languages (java, C, C++)
// More of a pattern matching statement

fn main() {
    let animal = "Duck";
    match animal {
        "Duck" => println!("Quack"),
        "Dog" => println!("Bark"),
        _ => println!("All quiet out here") // default behaviour
    }
    alternative_frequency();
    validate_frequency();
}


fn validate_frequency() {
    let ndb_freq:u16 = 384;
    match ndb_freq {
        200..=500 => {
            println!("NDB frequency is valid")
        }
        _ => {
            println!("Invalid ndb frequency")
        }
    }
}


fn alternative_frequency(){
    let ndb_freq:u16 = 384;

    match ndb_freq {
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            print!("NDB frequency is valid");
        },
        _ => {
            println!("NDB frequency is invalid");
        }
    }
}