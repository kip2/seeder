use std::{fs::File, io::Read};

use serde_json::{Error, Value};

fn main() {
    let mut file = File::open("data.json").expect("File not found");

    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");
    println!("Hello, world!");

    let v: Value = serde_json::from_str(&data).unwrap();

    println!("{:?}", v);
}
