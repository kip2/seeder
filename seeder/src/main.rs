use std::env;

use dotenv::dotenv;
use seeder::{
    db::{self, insert_data},
    json::*,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    let file_path = "data.json";
    let data = read_json_file(file_path).unwrap();

    insert_data(&pool, data).await.unwrap();
}
