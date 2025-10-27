use std::io::prelude::*;
use std::fs::File;
use clap::Parser;

/// Read the file contents
#[derive(Parser)]
struct Args {
    /// The file name in the current directory
    file: String
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut file = File::open(&args.file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}