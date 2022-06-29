mod git_command_inquire;

use inquire::Select;

const GIT: &str = "git";
const CONFIG: &str = "config";
const HELP: &str = "help";
const BUG_REPORT: &str = "bugreport";

pub fn setup_config_inquire() {
    let categories = vec![GIT, CONFIG, HELP, BUG_REPORT,];
    let category = Select::new("Please select a command:", categories).prompt();

    match category {
        Ok(category) => match_setup_config_category(category),
        Err(_) => println!("There was an error, please try again"),
    };
}

fn match_setup_config_category(category: &str) {
    match category {
        "git" => git_command_inquire::git_command_inquire(),
        // "config" => config_command_inquire(),
        // "help" => help_command_inquire(),
        // "bugreport" => bugreport_command_inquire(),
        &_ => todo!(),
    };
}