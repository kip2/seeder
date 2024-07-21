use seeder::db::insert;

#[tokio::main]
async fn main() {
    insert().await.expect("Failed INSERT query");
}
