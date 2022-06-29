mod setup_config_inquire;
use inquire::Select;

const SETUP_CONFIG: &str = "Setup and Config";
const GETTING_CREATING: &str = "Getting and Creating Projects";
const BASIC_SNAPSHOTTING: &str = "Basic Snapshotting";
const BRANCHING_MERGING: &str = "Branching and Merging";
const SHARING_UPDATING: &str = "Sharing and Updating Projects";
const INSPECTION_COMPARISON: &str = "Inspection and Comparison";
const PATCHING: &str = "Patching";
const DEBUGGING: &str = "Debugging";

pub fn git_category_inquire() {
    let categories = vec![
        SETUP_CONFIG,
        GETTING_CREATING,
        BASIC_SNAPSHOTTING,
        BRANCHING_MERGING,
        SHARING_UPDATING,
        INSPECTION_COMPARISON,
        PATCHING,
        DEBUGGING,
        "Exit"];

    let category = Select::new("Please select a category:", categories).prompt();

    match category {
        Ok(category) => match_git_category(category),
        Err(_) => println!("There was an error, please try again"),
    };
}

fn match_git_category(category: &str) {
    match category {
        SETUP_CONFIG => setup_config_inquire::setup_config_inquire(),
        "Getting and Creating Projects" => exit_program(),
        "Basic Snapshotting" => exit_program(),
        "Branching and Merging" => exit_program(),
        "Sharing and Updating Projects" => exit_program(),
        "Inspection and Comparison" => exit_program(),
        "Patching" => exit_program(),
        "Debugging" => exit_program(),
        "Exit" => exit_program(),
        &_ => todo!(),
    };
}

fn exit_program() {
    println!("Exitting program now, thank you for using Gust");
}