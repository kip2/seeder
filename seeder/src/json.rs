use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

#[derive(Deserialize, Debug, PartialEq)]
pub struct TableColumn {
    pub data_type: String,
    pub column_name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonData {
    pub table_name: String,
    pub table_columns: Vec<TableColumn>,
    pub table_rows: Vec<Vec<Value>>,
}

fn validate_row_column_count(data: &JsonData) -> bool {
    let column_count = data.table_columns.len();
    for (i, row) in data.table_rows.iter().enumerate() {
        if row.len() != column_count {
            return false;
        }
    }
    true
}

#[test]
fn test_validate_column_count() {
    let data = JsonData {
        table_name: "test_table".to_string(),
        table_columns: vec![
            TableColumn {
                data_type: "string".to_string(),
                column_name: "name".to_string(),
            },
            TableColumn {
                data_type: "int".to_string(),
                column_name: "age".to_string(),
            },
        ],
        table_rows: vec![
            vec![json!("Alice"), json!(30)],
            vec![json!("Bob"), json!(25)],
        ],
    };

    assert!(validate_row_column_count(&data));
}

#[test]
fn test_validate_column_count_failure() {
    let data = JsonData {
        table_name: "test_table".to_string(),
        table_columns: vec![
            TableColumn {
                data_type: "string".to_string(),
                column_name: "name".to_string(),
            },
            TableColumn {
                data_type: "int".to_string(),
                column_name: "age".to_string(),
            },
        ],
        table_rows: vec![
            vec![json!("Alice"), json!(30)],
            vec![json!("Bob")], // カラム数が一致しない
        ],
    };

    assert!(!validate_row_column_count(&data));
}

pub fn validate_columns_data_type(data: &JsonData) -> bool {
    let allowed_types = ["int", "string", "float", "date"];
    for column in &data.table_columns {
        if !allowed_types.contains(&column.data_type.as_str()) {
            return false;
        }
    }
    true
}
#[test]
fn test_validate_columns_data_type_success() {
    let data = JsonData {
        table_name: "test_table".to_string(),
        table_columns: vec![
            TableColumn {
                data_type: "string".to_string(),
                column_name: "name".to_string(),
            },
            TableColumn {
                data_type: "int".to_string(),
                column_name: "age".to_string(),
            },
            TableColumn {
                data_type: "float".to_string(),
                column_name: "salary".to_string(),
            },
            TableColumn {
                data_type: "date".to_string(),
                column_name: "birth_date".to_string(),
            },
        ],
        table_rows: vec![
            vec![
                serde_json::json!("Alice"),
                serde_json::json!(30),
                serde_json::json!(50000.0),
                serde_json::json!("1990-01-01"),
            ],
            vec![
                serde_json::json!("Bob"),
                serde_json::json!(25),
                serde_json::json!(60000.0),
                serde_json::json!("1995-05-15"),
            ],
        ],
    };

    assert!(validate_columns_data_type(&data));
}

#[test]
fn test_validate_columns_data_type_failure() {
    let data = JsonData {
        table_name: "test_table".to_string(),
        table_columns: vec![
            TableColumn {
                data_type: "string".to_string(),
                column_name: "name".to_string(),
            },
            TableColumn {
                data_type: "int".to_string(),
                column_name: "age".to_string(),
            },
            TableColumn {
                data_type: "double".to_string(),
                column_name: "salary".to_string(),
            }, // 不正なデータ型
            TableColumn {
                data_type: "date".to_string(),
                column_name: "birth_date".to_string(),
            },
        ],
        table_rows: vec![
            vec![
                serde_json::json!("Alice"),
                serde_json::json!(30),
                serde_json::json!(50000.0),
                serde_json::json!("1990-01-01"),
            ],
            vec![
                serde_json::json!("Bob"),
                serde_json::json!(25),
                serde_json::json!(60000.0),
                serde_json::json!("1995-05-15"),
            ],
        ],
    };

    assert!(!validate_columns_data_type(&data));
}

/// JSONファイルに設定されたインサート用のSQLデータを読み込む関数
///
/// JSONファイルのデータ構造は以下の通りである
///
/// ```json
/// {
///     "table_columns": [
///         {
///             "data_type": "string",
///             "column_name": "name",
///         },
///         {
///         "data_type": "int",
///         "column_name": "lifespan"
///         }
///     ]
///     ,
///     "table_rows": [
///         [
///         "Ryzen 9 5900X",
///         5
///         ],
///     ]
/// }
/// ```
///
pub fn read_json_file(file_path: &str) -> Result<JsonData, Box<dyn Error>> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader).expect("Failed JSON file.");

    Ok(data)
}

#[test]
fn test_read_json_file() {
    let file_path = "test/test.json";

    let expected_data = JsonData {
        table_name: "test_table".to_string(),
        table_columns: vec![
            TableColumn {
                data_type: "string".to_string(),
                column_name: "name".to_string(),
            },
            TableColumn {
                data_type: "string".to_string(),
                column_name: "type".to_string(),
            },
            TableColumn {
                data_type: "string".to_string(),
                column_name: "brand".to_string(),
            },
        ],
        table_rows: vec![
            vec![json!("Ryzen 9 5900X"), json!("CPU"), json!("AMD")],
            vec![json!("GeForce RTX 3080"), json!("GPU"), json!("NVIDIA")],
            vec![
                json!("Samusung 970 EVO SSD"),
                json!("SSD"),
                json!("Samsung"),
            ],
        ],
    };

    let result = read_json_file(&file_path).unwrap();

    assert_eq!(result, expected_data);
}

// 渡したjsonファイルについてのバリデーションをまとめて行う関数
//
// # Arguments
// ```
// json_file_path: &str
// ```
//
// jsonファイルのパス
//
// pub fn validate_json_data(json_file_path: &str) -> bool {
//     let (table_columns, table_row) = read_json_file(json_file_path);

//     if !validate_row_column_length(&table_columns, &table_row) {
//         return false;
//     };

//     if !validate_table_columns_type(&table_columns) {
//         return false;
//     };

//     true
// }

// カラムのデータタイプが全て、使用して良い型かどうかを判定する
//
// なお、使用して良い型かどうかはハードコードされたvariable_types
//
// - int
// - float
// - string
//
// JSONデータ側で使用する恣意的なデータ型であり、Rustの型と一致していないことに注意する
//
// # Arguments
// ```
// table_columns: &Option<Value>
// ```
//
// インサート用のテーブルカラムデータ
// 型はserde::json::Value
//
// # Return
//
// カラムデータに使用を許容されていないデータが入っていないかどうかを判定する
//
//
// pub fn validate_table_columns_type(table_columns: &Option<Value>) -> bool {
//     let variable_types = ["int", "string", "float"];

//     let table_columns_value = table_columns.as_ref().unwrap();
//     let columns_data = table_columns_value.as_array().unwrap();

//     for data in columns_data {
//         let data_type_str = data.get("data_type").unwrap().as_str().unwrap();
//         if !variable_types.contains(&data_type_str) {
//             return false;
//         }
//     }
//     true
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::json;

//     #[test]
//     fn test_validate_table_columns_type_valid() {
//         let table_columns = json!([
//             {"column_name":"name", "data_type":"string"},
//             {"column_name":"age", "data_type":"int"},
//             {"column_name":"salary", "data_type":"float"}
//         ]);
//         assert!(validate_table_columns_type(&Some(table_columns)));
//     }

//     #[test]
//     fn test_validate_table_columns_type_invalid_type() {
//         let table_columns = json!([
//             {"column_name":"name", "data_type":"string"},
//             // invalid type "integer"
//             {"column_name":"age", "data_type":"integer"},
//             {"column_name":"salary", "data_type":"float"}
//         ]);
//         assert!(!validate_table_columns_type(&Some(table_columns)));
//     }

//     #[test]
//     fn test_validate_table_columns_type_missing_data_type() {
//         let table_columns = json!([
//             {"column_name":"name", "data_type":"string"},
//             // missing data type
//             {"column_name":"age"},
//             {"column_name":"salary", "data_type":"float"}
//         ]);
//         let result = std::panic::catch_unwind(|| validate_table_columns_type(&Some(table_columns)));
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_validate_table_columns_type_empty_array() {
//         let table_columns = json!([]);
//         assert!(validate_table_columns_type(&Some(table_columns)));
//     }
// }

// カラムデータの個数と、ロウデータの個数が一致しているかをバリデーションする関数
//
// なお、DBのテーブルデータのカラム数と一致するかは判定しない
//
// # Arguments
//
// ```
// (table_columns: &Option<Value>, table_row: &Option<Value>)
// ```
// インサート用のテーブルカラムデータと、テーブルロウデータのタプルを受け取る
// 型はserde::json::Value
//
// # Returns
//
// カラムデータの個数と、全てのロウデータの個数が一致していればtrueを返す
//
// 一致していなければfalseを返す
//
// ```
// bool
// ```

// pub fn validate_row_column_length(
//     table_columns: &Option<Value>,
//     table_row: &Option<Value>,
// ) -> bool {
//     let table_columns_value = table_columns.as_ref().expect("table_columns not found");

//     let columns = table_columns_value
//         .as_array()
//         .expect("table_columns is not a valid array");

//     let columns_len = columns.len();

//     let table_row_values = table_row.as_ref().expect("table_row not found");

//     let rows = table_row_values
//         .as_array()
//         .expect("table_row is not a valid array");

//     let all_rows_have_same_length = rows
//         .iter()
//         .all(|row| row.as_array().expect("Each row should be an array").len() == columns_len);

//     if all_rows_have_same_length {
//         true
//     } else {
//         false
//     }
// }
