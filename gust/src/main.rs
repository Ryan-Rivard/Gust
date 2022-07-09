mod git_stuff;
mod prompter;
mod executor;

use clap::Parser;

#[derive(Parser)]
struct Args {
    command: Option<String>,
}

fn main() {
    let args = Args::parse();

    let command = get_git_category_commands();

    executor::execute_git_command(vec![command]);

    println!("Pleasure doing business with you.");
}

fn get_git_category_commands() -> &'static str{

    let message = git_stuff::get_message();
    let options = git_stuff::get_options();
    let selection = prompter::prompt_selection(message, options);
    return git_stuff::process_selections(selection)
}

fn specific_thread(command: String) {
    if command.eq("start") {
        println!("Starting up Gust - we should check config options");
    }
}

