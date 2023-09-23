fn main() {
    let animal = "Duck"; 

    if let animal = "Duck" { // works only if the statement afer let is irrefutable
        println!("Quack")
    }
}
