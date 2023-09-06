#![allow(unused_variables)]

fn main() {
    let unused_variable: u32 = 0;
    let location: [f64; 10_000] = [0.0; 10_000];
    let location_tuple: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    let (name, latitude, longitude) = location_tuple;
    println!("Location name: {}, latitude: {}, longitude: {}",
    location_tuple.0, location_tuple.1, location_tuple.2);
    println!("Location name: {}, latitude: {}, longitude: {}",
    name, latitude, longitude);
    concat_str_slice();
    casting_variables();
    scope_example();
}


fn convert_str_slice() {
    let person_name_string = String::from("Donald Mallard");
    let person_name_slice = &person_name_string; //dereferencing a variable and actually points to the heap memory address
    let person_name_slice2 = person_name_string.as_str(); //& and * are the 2 characters which deal with memory pointers
}


fn concat_str_slice() {
    let duck = "Duck";
    let airlines = "Airlines";
    let airline_name = [duck, " ", airlines].concat(); //note that the concat produces a String out of string slices
    println!("{}", airline_name); //exclamation mark denotes macros
    println!("{}", format!("{} {}", duck, airlines));
    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' '); //note that push expects a char which is passed with single quotes. Otherwise, compile error
    slogan = slogan + "every time";
    println!("{}", slogan);
}


fn casting_variables() {
    let float_a: f32 = 17.2;
    let unsigned_b: u8 = 5;
    let float_b: f32 = unsigned_b as f32;

    let result = float_a / float_b;
}


fn scope_example() {
    let scope_test = "outer scope";
    println!("{}", scope_test);
    // defines a scope/code block
    {
        let scope_test = "inner scope"; //shadowed within the code block
        println!("{}", scope_test);
    }
    println!("{}", scope_test);
}
