use seeder::{
    db::{self, insert_row},
    json::*,
};

#[tokio::main]
async fn main() {
    // validate_json_data("data.json");
    insert_row().await;
}
