use clap::Parser;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

use tloc::traverse_dir;

#[derive(Parser)]
#[command(name = "tloc")]
#[command(about = "a simple solidity codebase parser")]
struct Args {
    #[arg(long, short)]
    path: Option<PathBuf>,
    #[arg(long, short)]
    out: Option<PathBuf>,
    #[arg(long)]
    no_recurse: bool,
}

fn main() {
    let args = Args::parse();

    let path = match args.path {
        Some(root) => root,
        None => PathBuf::from("./"),
    };

    let out = match args.out {
        Some(location) => location,
        None => PathBuf::from("scope.md"),
    };

    let mut file = OpenOptions::new()
        .write(true)        // Enable append mode
        .create(true)       // Create the file if it doesn't exist
        .truncate(true)     // Wipe file if already exists
        .open(out).unwrap();     // Open the file, returning a Result, unwrap it
    
    writeln!(file, "# Scope").unwrap();

    if !path.is_file() && !path.is_dir() {
        panic!("Incorrect Path Provided");
    }

    let total_code_count = traverse_dir(&path, &mut file, &!args.no_recurse, 2);

    writeln!(file, "TOTAL: {}", total_code_count).unwrap();
    println!("Scope complete, total lines {total_code_count}");
}

