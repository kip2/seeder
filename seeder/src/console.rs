use clap::Parser;
use std::error::Error;

use crate::{
    db::{insert, insert_random_data},
    json::create_template_json_file,
};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        short = 'f',
        long = "filepath",
        help = "File path for executing seeds to the database. You can specify multiple file paths.",
        num_args = 1..
    )]
    file_paths: Vec<String>,

    #[arg(
        short = 'r',
        long = "random",
        help = "Generate and seed the database with the specified number of random data entries. Provide the file path and the number of entries.",
        value_names = &["FILE_PATH", "NUMBER"]
    )]
    random: Option<Vec<String>>,

    #[arg(
        short = 'c',
        long = "create",
        help = "Create a new JSON file with the specified structure.",
        value_names = [&"FILE_PATH"]
    )]
    create: Option<String>,
}

/// コンソール処理を実行する
///
pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.file_paths.len() > 0 {
        // run insert
        run_queries(&args).await?;
    } else if let Some(random_args) = args.random {
        if random_args.len() == 2 {
            // run insert random data
            run_insert_random_data(&random_args).await?;
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The --random option requires exactly 2 arguments: <file_path> and <number>",
            )));
        }
    } else if let Some(create_args) = args.create {
        // run create template JSON file
        run_create_template_json_file(&create_args)?;
    }
    Ok(())
}

/// テンプレートファイルをクリエイトする関数のラッパー関数
///
fn run_create_template_json_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("===========");
    println!("Starting create template JSON file: {}", &file_path);
    create_template_json_file(&file_path)?;
    println!("Finished create template JSON file: {}", &file_path);
    println!("===========");
    Ok(())
}

/// 指定された数のランダムデータを生成し、DBにINSERTを行う
///
async fn run_insert_random_data(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let file_path = &args[0];
    let n: usize = args[1].parse().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid number format for random data entries",
        )
    })?;

    println!("===========");
    println!(
        "Starting random data insert execution for file: {}",
        &file_path
    );

    insert_random_data(&file_path, n).await?;

    println!(
        "Finished random data insert execution for file: {}",
        &file_path
    );
    println!("===========");

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
