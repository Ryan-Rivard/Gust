use inquire::Select;
use std::process::Command;


pub fn git_command_inquire() {
    let commands = vec![
        "version",
        "executable path",
        "html documentation path",
        "manual path",
        "info path"];
    let cmd = Select::new("Please select an argument:", commands).prompt();

    match cmd {
        Ok(cmd) => match_git_command(cmd),
        Err(_) => println!("There was an error, please try again"),
    };
}

fn match_git_command(cmd: &str) {
    match cmd {
        "version" => execute_command_version(),
        // "executable path" => execute_command_exec_path(),
        // "html documentation path" => execute_command_html_path(),
        // "manual path" => execute_command_man_path(),
        // "info path" => execute_command_info_path(),
        &_ => todo!(),
    };
}

fn execute_command_version() {
    println!("Executing Command: git version");
    let command = 
        match Command::new("git")
            .arg("version")
            .output() {
                Ok(val) => val,
                Err(e) => panic!("Error: {}", e),
            };

    println!("{}", String::from_utf8_lossy(&command.stdout));
}