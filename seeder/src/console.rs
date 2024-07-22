use clap::Parser;
use std::error::Error;

use crate::db::insert;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        short = 'f',
        long = "filepath",
        help = "File path for executing seeds to the database"
    )]
    file_path: String,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let file_path = &args.file_path;

    insert(file_path).await?;

    Ok(())
}
