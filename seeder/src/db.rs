use dotenv::dotenv;
use sqlx::any::{install_default_drivers, AnyQueryResult};
use sqlx::{AnyPool, Row};
use std::{env, error::Error, fs};

async fn db_pool() -> Result<AnyPool, Box<dyn Error>> {
    dotenv().expect("Fialed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DABASE_URL must be set");

    install_default_drivers();

    let pool = AnyPool::connect(&database_url).await?;
    Ok(pool)
}
