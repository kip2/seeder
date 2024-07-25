use seeder::console::run;
use seeder::json::generate_random_data;
use std::process;

#[tokio::main]
async fn main() {
    let n = 2;
    let data = generate_random_data("column.json", n);
    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    // if let Err(e) = run().await {
    //     eprintln!("Error: {}", e);
    //     process::exit(1);
    // };
}
