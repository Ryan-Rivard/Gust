use inquire::MultiSelect;
use inquire::formatter::MultiOptionFormatter;
use std::fmt::Debug;
use std::process::Command;

pub fn help_all_command_inquire() {
    let options = vec!["no external commands","no aliases", "verbose"];

    let formatter: MultiOptionFormatter<&str> = &|a| format!("{} different fruits", a.len());

    let ans = MultiSelect::new("Select the fruits for your shopping list:", options)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(options) => execute_command_all(options),
        Err(_) => println!("The shopping list could not be processed"),
    };
}

fn execute_command_all(options: Vec<&str>) {
    println!("Executing Command: git help all");

    let mut command = Command::new("git");
    command.arg("help");
    command.arg("--all");

    for option in options {
        let myoption = match option {
            "no external commands" => continue,
            "no aliases" => "--alias",
            "verbose" => "--verbose",
            &_ => todo!()
        };
        command.arg(myoption);
    }

    let myargs = command.get_args();

    for lol in myargs {
        println!("{}",lol.to_string_lossy());
    }

    let command = command.output(); 
    
    let command = match command {
        Ok(val) => val,
        Err(e) => panic!("Error: {}", e),
    };

    println!("{}", String::from_utf8_lossy(&command.stdout));
}