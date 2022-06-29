mod git_category_inquire;

use clap::Parser;

#[derive(Parser)]
struct Args {
    command: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(command) => specific_thread(command),
        None => git_category_inquire::git_category_inquire(),
    }

    println!("Pleasure doing business with you.");
}

fn specific_thread(command: String) {
    if command.eq("start") {
        println!("Starting up Gust - we should check config options");
    }
}