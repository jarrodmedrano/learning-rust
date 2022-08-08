#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    let location2 = ("KCLE", 41.4094069, -81.8546911);
    let (name, latitude, longitude) = location2;
    println!("location name: {}, latitude: {}, longitude: {}", name, latitude, longitude);
    let location: [f32;2] = [0.0, 0.0];


    //string slice
    let person_name_slice = "Donald Mallard";

    //string stored on the heap
    let person_name_string2 = String::from("Donald Mallard").to_string();
    let person_name_string3 = "donald mallard".to_string();

    println!("{}", person_name_slice);
    println!("{}", person_name_string2);
    println!("{}", person_name_string3);
}


