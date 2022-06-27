use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Args {
    command: String,
}

fn main() {
    let args = Args::parse();

    let mut git_arg = "--".to_owned();
    git_arg.push_str(&args.command);

    let command = 
        match Command::new("git")
            .arg(git_arg)
            .output() {
                Ok(val) => val,
                Err(e) => panic!("Error: {}", e),
            };

    println!("{}", String::from_utf8_lossy(&command.stdout));
}
