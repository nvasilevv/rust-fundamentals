#![allow(unused_variables)]

fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0; // an immutable variable can be shadowed effectively changing its value
    // a const cannot be changed

    let kcle_latitude_degrees: f64 = 41.4075;
    let kcle_longitude_degrees: f64 = -81.581111;

    let kslc_latitude_degress: f64 = 40.7861;
    let kslc_longitude_degrees: f64 = -111.9822;

    let kcle_latitude_radians: f64 = kcle_latitude_degrees.to_radians();
    let kslc_latitude_radians: f64 = kslc_latitude_degress.to_radians();

    let delta_latitude: f64 = (kcle_latitude_degrees - kslc_latitude_degress).to_radians();
    let delta_longitude: f64 = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(),2)
        + kcle_latitude_radians.cos()
        * kslc_latitude_radians.cos()
        * f64::powi((delta_longitude / 2.0).sin(),2);

    let central_angle: f64 = 2.0 * inner_central_angle.sqrt().asin();

    let distance: f64 = EARTH_RADIUS_IN_KILOMETERS * central_angle;

    println!("The distance between the two points is {:.1} kilometers", distance);

    main_2();
}

fn main_2() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0; // an immutable variable can be shadowed effectively changing its value

    let route = [
        ("KCLE", 41.4075, -81.851111),
        ("LEYIR", 41.51030, -83.88080),
        ("PIONS", 41.65390, -84.48190)
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians(); //1 refers to the argument of the tuple
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(),2)
                    + previous_waypoint_radians.cos()
                    * waypoint_radians.cos()
                    * f64::powi((delta_longitude / 2.0).sin(),2);
                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());
                println!("The distance between {} and {} is {:.1} kilometers", previous_waypoint_value.0, waypoint.0, distance);
            }
        }
    }
}
