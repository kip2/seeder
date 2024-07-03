use std::{fs::File, io::Read};

use serde_json::Value;

fn main() {
    let mut file = File::open("data.json").expect("File not found");

    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");
    println!("Hello, world!");

    let v: Value = serde_json::from_str(&data).unwrap();

    let mut table_columns: Option<Value> = None;
    let mut table_row: Option<Value> = None;

    if let Value::Object(map) = v {
        if let Some(table_columns_value) = map.get("table_columns") {
            table_columns = Some(table_columns_value.clone());
        } else {
            println!("table_columns not found");
        }

        if let Some(table_row_value) = map.get("table_row") {
            table_row = Some(table_row_value.clone());
        } else {
            println!("table_row not found");
        }
    } else {
        println!("Expected a JSON object");
    }

    if let Some(tc) = &table_columns {
        println!("table_columns: {:?}", tc);
    }
    if let Some(tr) = &table_row {
        println!("table_row: {:?}", tr);
    }
}
