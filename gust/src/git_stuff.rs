use core::panic;

const VERSION: &str = "version";
const HELP: &str = "help";
const EXEC_PATH: &str = "executable path";
const HTML_PATH: &str = "html path";
const MAN_PATH: &str = "manual path";
const INFO_PATH: &str = "info path";


pub fn get_message() -> &'static str {
    "please select the arguments:"
}

pub fn get_options() -> Vec<&'static str> {
    vec![VERSION,
        HELP,
        EXEC_PATH,
        HTML_PATH,
        MAN_PATH,
        INFO_PATH]
}

pub fn process_selections(selection: &'static str) -> &'static str {
    match selection {
        VERSION => "--version",
        HELP => "--help",
        EXEC_PATH => "--exec-path",
        HTML_PATH => "--html-path",
        MAN_PATH => "--man-path",
        INFO_PATH => "--info-path",
        &_ => panic!("oh nooo"),
    }
}