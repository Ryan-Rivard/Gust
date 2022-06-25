#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// message to pass to the terminal
    message: String,
}

fn main() {
    let args = Cli::parse();

    println!("{}", &args.message);
}
