use core::num;
use dotenv::dotenv;
use sqlx::any::{install_default_drivers, AnyQueryResult};
use sqlx::{database, query, AnyPool, Row};
use std::{env, error::Error, fs};

use crate::json::{self};

// pub async fn insert_row() {
//     let db = db_pool().await.unwrap();

//     // todo: テーブルネームをJSONデータから取得する関数に置き換える
//     let table_name = "table_name";

//     let (table_columns, table_rows) = json::read_json_file("data.json");
//     let (table_columns, table_rows) = tuple_to_string_vector(&table_columns, &table_rows);

//     insert(&db, table_name, (table_columns, table_rows)).await;
// }

// async fn db_pool() -> Result<AnyPool, Box<dyn Error>> {
//     dotenv().expect("Fialed to read .env file");
//     let database_url = env::var("DATABASE_URL").expect("DABASE_URL must be set");

//     install_default_drivers();

//     let pool = AnyPool::connect(&database_url).await?;
//     Ok(pool)
// }

// fn generate_placeholder(
//     database_url: &str,
//     num_placeholders: usize,
// ) -> Result<String, Box<dyn Error>> {
//     match database_url {
//         url if url.starts_with("postgres://") => Ok((1..=num_placeholders)
//             .map(|i| format!("${}", i))
//             .collect::<Vec<String>>()
//             .join(", ")),
//         url if url.starts_with("mysql://") => Ok(std::iter::repeat("?")
//             .take(num_placeholders)
//             .collect::<Vec<&str>>()
//             .join(", ")),
//         _ => Err("Unsupported database URL".into()),
//     }
// }

// pub async fn insert(
//     db: &AnyPool,
//     table_name: &str,
//     (table_columns, table_rows): (Vec<String>, Vec<String>),
// ) -> Result<AnyQueryResult, Box<dyn Error>> {
//     let database_url = env::var("DATABASE_URL").expect("DATBASE_URL must be set");

//     let placeholder = generate_placeholder(&database_url, table_columns.len()).unwrap();

//     let query = format!(
//         "INSERT INTO {} ({}) VALUES ({})",
//         table_name,
//         table_columns.join(", "),
//         placeholder
//     );

//     let mut query_builder = sqlx::query(&query);

//     for value in table_rows {
//         query_builder = query_builder.bind(value);
//     }

//     let result = query_builder.execute(db).await;
//     result.map_err(|e| e.into())
// }
