#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    let location2 = ("KCLE", 41.4094069, -81.8546911);
    let (name, latitude, longitude) = location2;
    println!("location name: {}, latitude: {}, longitude: {}", name, latitude, longitude);
    let location: [f32;2] = [0.0, 0.0];
}


