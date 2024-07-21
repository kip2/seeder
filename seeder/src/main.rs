use std::{env, error::Error};

use dotenv::dotenv;
use seeder::{
    db::{self, insert_data},
    json::*,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    let file_path = "data.json";
    let data = read_json_file(file_path).unwrap();

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
