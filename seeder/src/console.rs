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
        insert(file_path).await?;
    }

    Ok(())
}
