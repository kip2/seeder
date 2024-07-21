use seeder::db::insert;

#[tokio::main]
async fn main() {
    if let Err(e) = insert().await {
        eprintln!("Error: {}", e);
    };
}
