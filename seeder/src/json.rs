use fake::faker::chrono::raw::DateTime;
use fake::faker::company::en::*;
use fake::faker::lorem::en::*;
use fake::locales::*;
use fake::Fake;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use std::{error::Error, fs::File, io::BufReader};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct TableColumn {
    pub data_type: String,
    pub column_name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct JsonData {
    pub table_name: String,
    pub table_columns: Vec<TableColumn>,
    pub table_rows: Vec<Vec<Value>>,
}

pub fn generate_random_data(file_path: &str) -> JsonData {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let mut data: JsonData = serde_json::from_reader(reader).expect("Failed JSON file.");

    let mut table_rows = Vec::new();

    for _ in 0..10 {
        let row = vec![
            json!(CompanyName().fake::<String>()),
            json!(CompanySuffix().fake::<String>()),
            json!(CompanyName().fake::<String>()),
            json!(CompanySuffix().fake::<String>()),
            json!(DateTime(EN).fake::<String>()),
            json!(Sentence(1..2).fake::<String>()),
            json!(rand::thread_rng().gen_range(50..101)),
            json!(rand::thread_rng().gen_range(10.0..1000.0)),
            json!(rand::thread_rng().gen_range(0.01..0.1)),
            json!(rand::thread_rng().gen_range(50.0..500.0)),
            json!(rand::thread_rng().gen_range(0.01..1.0)),
            json!(rand::thread_rng().gen_range(0.01..1.0)),
            json!(rand::thread_rng().gen_range(0.01..1.0)),
            json!(rand::thread_rng().gen_range(1..10)),
        ];
        table_rows.push(row);
    }

    data.table_rows = table_rows;
    data
}

/// JSONファイルに関数バリデーションをまとめて行う関数
///
/// 行われるバリデーションは以下の2つ
///
/// - validate_row_column_count()
/// カラムの数と、各ロウデータの数が一致しているかを検証
///
/// - validate_columns_data_type()
/// カラムのデータタイプが、許可されたデータ型であるかを検証する
///
pub fn validate_json_data(data: &JsonData) -> Result<(), String> {
    // カラムの数と、各ロウデータの数が一致しているかを検証
    if !validate_row_column_count(&data) {
        return Err("Row column count validation failed.".into());
    }

    // カラムのデータタイプが許可されたデータ型であるかを検証
    if !validate_columns_data_type(&data) {
        return Err("Column data type validation failed.".into());
    }

    Ok(())
}

/// JSONファイルの、カラムデータの数と各ロウデータの数が一致しているかを検証する
///
fn validate_row_column_count(data: &JsonData) -> bool {
    let column_count = data.table_columns.len();

    for (_, row) in data.table_rows.iter().enumerate() {
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

/// JSONファイルのカラムのデータ型が、許可されたデータ型であるかを検証する
///
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
///     "table_name": "computer_parts",
///     "table_columns": [
///         {
///             "data_type": "string",
///             "column_name": "name",
///         },
///         {
///             "data_type": "int",
///             "column_name": "lifespan"
///         }
///     ]
///     ,
///     "table_rows": [
///         [
///             "Ryzen 9 5900X",
///             5
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
