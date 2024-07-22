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

/// コンソール処理を実行する
///
pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // run insert
    run_queries(&args).await?;
    Ok(())
}

/// クエリのインサート処理を実行する
///
async fn run_queries(args: &Args) -> Result<(), Box<dyn Error>> {
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
