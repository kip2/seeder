use crate::json::*;
use dotenv::dotenv;
use serde_json::Value;
use sqlx::{PgPool, Pool, Postgres, Transaction};
use std::{env, error::Error};

/// ファイルパスに記載されたseed用のデータで、INSERT処理を実行する
///
pub async fn insert(file_path: &str) -> Result<(), Box<dyn Error>> {
    let pool = generate_db_connection().await;

    let data = read_json_file(file_path).unwrap();

    // validation
    if let Err(e) = validate_json_data(&data) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e,
        )));
    }

    let mut transaction = pool.begin().await.unwrap();

    match insert_data(&mut transaction, data).await {
        Ok(_) => {
            transaction.commit().await.unwrap();
        }
        Err(e) => {
            transaction.rollback().await.unwrap();
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };
    Ok(())
}

/// データをランダムに生成して、DBにインサートする処理を行う
///
pub async fn insert_random_data(file_path: &str, n: usize) -> Result<(), Box<dyn Error>> {
    let pool = generate_db_connection().await;

    let data = generate_random_data(&file_path, n);

    // validation
    if !validate_columns_data_type(&data) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid data type found in table columns",
        )));
    }

    let mut transaction = pool.begin().await.unwrap();

    match insert_data(&mut transaction, data).await {
        Ok(_) => {
            transaction.commit().await.unwrap();
        }
        Err(e) => {
            transaction.rollback().await.unwrap();
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };
    Ok(())
}

/// DBコネクションを生成する処理を行う
///
async fn generate_db_connection() -> Pool<Postgres> {
    dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    pool
}

/// クエリ用のデータが定義されたJsonDataを受け取り、INSERT処理を行う
///
/// ## 備忘
///
/// postgreSQLは、型が"date"の場合にはバインドメッセージの変換の必要があるため、変換を行なっている
pub async fn insert_data<'a>(
    transaction: &mut Transaction<'a, sqlx::Postgres>,
    data: JsonData,
) -> Result<(), sqlx::Error> {
    let columns: Vec<String> = data
        .table_columns
        .iter()
        .map(|col| col.column_name.clone())
        .collect();

    let columns_str = columns.join(", ");

    let table_name = data.table_name;

    let mut placeholders: Vec<String> = vec![];

    for (i, col) in data.table_columns.iter().enumerate() {
        if col.data_type == "date" {
            placeholders.push(format!("CAST(${} AS DATE)", i + 1));
        } else {
            placeholders.push(format!("${}", i + 1));
        }
    }
    let placeholders_str = placeholders.join(", ");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name, columns_str, placeholders_str
    );

    for row in data.table_rows {
        let mut query_builder = sqlx::query(&query);
        for (i, value) in row.iter().enumerate() {
            match data.table_columns[i].data_type.as_str() {
                "string" => {
                    if let Value::String(val) = value {
                        query_builder = query_builder.bind(val);
                    }
                }
                "int" => {
                    if let Value::Number(val) = value {
                        if let Some(int_val) = val.as_i64() {
                            query_builder = query_builder.bind(int_val);
                        }
                    }
                }
                "float" => {
                    if let Value::Number(val) = value {
                        if let Some(float_val) = val.as_f64() {
                            query_builder = query_builder.bind(float_val);
                        }
                    }
                }
                "date" => {
                    if let Value::String(val) = value {
                        query_builder = query_builder.bind(val);
                    }
                }
                _ => {}
            }
        }
        query_builder.execute(&mut **transaction).await?;
    }

    Ok(())
}
