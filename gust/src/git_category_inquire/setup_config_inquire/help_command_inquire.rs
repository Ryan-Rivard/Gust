mod help_all_command_inquire;

use inquire::Select;
use std::process::Command;


pub fn help_command_inquire() {
    let commands = vec![
        "no argument",
        "all",
        "config",
        "guides",
        "info"];
    let cmd = Select::new("Please select an argument:", commands).prompt();

    match cmd {
        Ok(cmd) => match_git_command(cmd),
        Err(_) => println!("There was an error, please try again"),
    };
}

fn match_git_command(cmd: &str) {
    match cmd {
        "no argument" => execute_command(),
        "all" => help_all_command_inquire::help_all_command_inquire(),
        // "html documentation path" => execute_command_html_path(),
        // "manual path" => execute_command_man_path(),
        // "info path" => execute_command_info_path(),
        &_ => todo!(),
    };
}

fn execute_command() {
    println!("Executing Command: git help");
    let command = 
        match Command::new("git")
            .arg("help")
            .output() {
                Ok(val) => val,
                Err(e) => panic!("Error: {}", e),
            };

    println!("{}", String::from_utf8_lossy(&command.stdout));
}