use clap::Parser;
use std::process::Command;
use inquire::Select;

#[derive(Parser)]
struct Args {
    command: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(command) => specific_thread(command),
        None => main_thread(),
    }

    println!("Pleasure doing business with you.");
}

fn main_thread() {
    let options = vec!["help", "version"];

    let ans = Select::new("What would you like to do?", options).prompt();

    match ans {
        Ok(choice) => exec_git_command(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn specific_thread(command: String) {
    if command.eq("start") {
        println!("Starting up Gust - we should check config options");
    }
}

fn exec_git_command(choice: &str) {
    println!("{}! lets do it!", choice);

    let mut git_arg = "--".to_owned();
    git_arg.push_str(choice);

    let command = 
        match Command::new("git")
            .arg(git_arg)
            .output() {
                Ok(val) => val,
                Err(e) => panic!("Error: {}", e),
            };

    println!("{}", String::from_utf8_lossy(&command.stdout));
}