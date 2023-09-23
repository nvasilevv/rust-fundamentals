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
    exponent_operator();
    bitwise_operator();
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


fn exponent_operator() {
    let squared = i32::pow(8, 2);
    let float_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.14);
    println!("integer: {},", squared);
    println!("float to int: {}", float_integer);
    println!("float to float: {}", float_float);
}


fn bitwise_operator() {
    let bitwise_and = 86 & 27;
    println!("bitwise and: {}", bitwise_and);

    let bitwise_or = 86 | 27;
    println!("bitwise or: {}", bitwise_or);

    let bitwise_xor = 86 ^ 27;
    println!("bitwise xor: {}", bitwise_xor);

    let left_shift = 86<<1; // this is equivalent to 86 * 2
    println!("left_shift: {}", left_shift);

    let right_shift = 86>>1; // this is equivalent to 86 / 2
    println!("right shift: {}", right_shift);
}
