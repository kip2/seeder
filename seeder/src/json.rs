use serde_json::Value;
use std::{fs::File, io::Read};

/// 渡したjsonファイルについてのバリデーションをまとめて行う関数
///
/// # Arguments
/// ```
/// json_file_path: &str
/// ```
///
/// jsonファイルのパス
///
pub fn validate_json_data(json_file_path: &str) -> bool {
    let (table_columns, table_row) = read_json_file(json_file_path);

    if !validate_row_column_length(&table_columns, &table_row) {
        return false;
    };

    if !validate_table_columns_type(&table_columns) {
        return false;
    };

    true
}

/// JSONから読み込んだタプルを、Vec<String>に変換する関数
///
/// # Arguments
/// ```
/// table_columns: &Option<Value>, table_row: &Option<Value>
/// ```
///
/// # Return
/// ```
/// table_columns: Vec<String>, table_row: Vec<String>
/// ```
///
fn tuple_to_string_vector(
    table_columns: &Option<Value>,
    table_row: &Option<Value>,
) -> (Vec<String>, Vec<String>) {
    let column_vec = match table_columns {
        Some(value) => vec![value.to_string()],
        None => vec!["None".to_string()],
    };

    let row_vec = match table_row {
        Some(value) => vec![value.to_string()],
        None => vec!["None".to_string()],
    };

    (column_vec, row_vec)
}

/// カラムのデータタイプが全て、使用して良い型かどうかを判定する
///
/// なお、使用して良い型かどうかはハードコードされたvariable_types
///
/// - int
/// - float
/// - string
///
/// JSONデータ側で使用する恣意的なデータ型であり、Rustの型と一致していないことに注意する
///
/// # Arguments
/// ```
/// table_columns: &Option<Value>
/// ```
///
/// インサート用のテーブルカラムデータ
/// 型はserde::json::Value
///
/// # Return
///
/// カラムデータに使用を許容されていないデータが入っていないかどうかを判定する
///
///
pub fn validate_table_columns_type(table_columns: &Option<Value>) -> bool {
    let variable_types = ["int", "string", "float"];

    let table_columns_value = table_columns.as_ref().unwrap();
    let columns_data = table_columns_value.as_array().unwrap();

    for data in columns_data {
        let data_type_str = data.get("data_type").unwrap().as_str().unwrap();
        if !variable_types.contains(&data_type_str) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_table_columns_type_valid() {
        let table_columns = json!([
            {"column_name":"name", "data_type":"string"},
            {"column_name":"age", "data_type":"int"},
            {"column_name":"salary", "data_type":"float"}
        ]);
        assert!(validate_table_columns_type(&Some(table_columns)));
    }

    #[test]
    fn test_validate_table_columns_type_invalid_type() {
        let table_columns = json!([
            {"column_name":"name", "data_type":"string"},
            // invalid type "integer"
            {"column_name":"age", "data_type":"integer"},
            {"column_name":"salary", "data_type":"float"}
        ]);
        assert!(!validate_table_columns_type(&Some(table_columns)));
    }

    #[test]
    fn test_validate_table_columns_type_missing_data_type() {
        let table_columns = json!([
            {"column_name":"name", "data_type":"string"},
            // missing data type
            {"column_name":"age"},
            {"column_name":"salary", "data_type":"float"}
        ]);
        let result = std::panic::catch_unwind(|| validate_table_columns_type(&Some(table_columns)));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_table_columns_type_empty_array() {
        let table_columns = json!([]);
        assert!(validate_table_columns_type(&Some(table_columns)));
    }
}

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
pub fn validate_row_column_length(
    table_columns: &Option<Value>,
    table_row: &Option<Value>,
) -> bool {
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
/// JSONファイルから取得したテーブルカラムとテーブルロウの2つをタプルにして返す
/// 型はserde::json::Value
///
pub fn read_json_file(file_path: &str) -> (Option<Value>, Option<Value>) {
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
