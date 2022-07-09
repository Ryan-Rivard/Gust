use std::process::Command;

pub fn execute_git_command(options: Vec<&str>) {
    let mut command = Command::new("git");

    for option in options {
        println!("{}", option);
        command.arg(option);
    }

    let command = command.output(); 
    
    let command = match command {
        Ok(val) => val,
        Err(e) => panic!("Error: {}", e),
    };

    println!("{}", String::from_utf8_lossy(&command.stdout));
}