fn main() {
    let word = "Dog";
    if word == "Duck" {
        println!("Quack");
    } else if word == "Dog" {
        println!("Bark");
    } else {
        println!("All quiet out here");
    };

    more_complex_if();
}


fn more_complex_if() {
    let available_aircraft = "Boeing";
    let minimum_crew = 7;
    let availabe_crew = 4;


    if (available_aircraft == "Boeing"
        || available_aircraft == "Airbus") // and conditions are evaluated first
        && minimum_crew <= availabe_crew {
            println!("okay");
        }
}