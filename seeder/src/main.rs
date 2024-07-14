use std::{fs::File, io::Read};

use serde_json::Value;

fn main() {
    let (table_columns, table_row) = read_json_file("data.json");

    let tf = validate_row_column_length(&table_columns, &table_row);

    println!("{}", tf);
}

fn validate_rows(table_row: &Option<Value>) -> bool {}

/// カラムデータの個数と、ロウデータの個数が一致しているかをバリデーションする関数
///
/// なお、DBのテーブルデータのカラム数と一致するかは判定しない
///
/// # Arguments
///
/// ```
/// (table_columns: &Option<Value>, table_row: &Option<Value>)
/// ```
/// インサート用のテーブルカラムデータと、テーブルロウデータのタプルを受け取る
/// 型はserde::json::Value
///
/// # Returns
///
/// カラムデータの個数と、全てのロウデータの個数が一致していればtrueを返す
///
/// 一致していなければfalseを返す
///
/// ```
/// bool
/// ```
///
fn validate_row_column_length(table_columns: &Option<Value>, table_row: &Option<Value>) -> bool {
    let table_columns_value = table_columns.as_ref().expect("table_columns not found");

    let columns = table_columns_value
        .as_array()
        .expect("table_columns is not a valid array");

    let columns_len = columns.len();

    let table_row_values = table_row.as_ref().expect("table_row not found");

    let rows = table_row_values
        .as_array()
        .expect("table_row is not a valid array");

    let all_rows_have_same_length = rows
        .iter()
        .all(|row| row.as_array().expect("Each row should be an array").len() == columns_len);

    if all_rows_have_same_length {
        true
    } else {
        false
    }
}

/// JSONファイルに設定されたインサート用のSQLデータを読み込む関数
///
/// # Arguments
/// ```
/// file_path: &str
/// ```
/// 読み込む対象のJSONファイルパス
///
/// # Returns
///
/// ```
/// (table_columns: Option<Value>, table_row: Option<Value>)
/// ```
///
/// JSONファイルのカラムデータのタプル。
/// table_columnsとtable_rowの2つをタプルにして返す
/// 型はserde::json::Value
///
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
