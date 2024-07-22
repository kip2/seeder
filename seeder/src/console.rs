use clap::Parser;
use std::error::Error;

use crate::db::insert;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        short = 'f',
        long = "filepath",
        help = "File path for executing seeds to the database",
        num_args = 1..
    )]
    file_paths: Vec<String>,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    for file_path in &args.file_paths {
        println!("===========");
        println!("Starting SQL execution for file: {}", &file_path);
        if let Err(e) = insert(file_path).await {
            eprintln!(
                "Failed to execute SQL for file: {}. Error: {}",
                file_path, e
            );
            eprintln!("===========");
            return Err(e);
        }
        println!("Finished SQL execution for file: {}", &file_path);
        println!("===========");
    }

    Ok(())
}
