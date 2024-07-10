use std::{fs::File, io::Read};

use serde_json::Value;

fn main() {
    let (table_columns, table_row) = read_json_file("data.json");

    let table_columns_value = table_columns.expect("table_columns not found");

    let columns = table_columns_value
        .as_array()
        .expect("table_columns is not a valid array");

    let columns_len = columns.len();
    println!("Length of table_clumns: {}", columns_len);

    let table_row_values = table_row.expect("table_row not found");

    let rows = table_row_values
        .as_array()
        .expect("table_row is not a valid array");

    let all_rows_have_same_length = rows
        .iter()
        .all(|row| row.as_array().expect("Each row should be an array").len() == columns_len);

    if all_rows_have_same_length {
        println!("All rows have the same number of elements as table_columns.");
    } else {
        println!("One or more rows do not have the same number of elements as table_columns.");
    }
}

fn read_json_file(file_path: &str) -> (Option<Value>, Option<Value>) {
    let mut file = File::open(file_path).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");

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

    (table_columns, table_row)
}
