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
}
