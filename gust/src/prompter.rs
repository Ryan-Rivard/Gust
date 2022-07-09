use inquire::{Select, MultiSelect};

pub fn prompt_selection(message: &str, options: Vec<&'static str>) -> &'static str {
    return match Select::new(message, options).prompt() {
        Ok(sel) => sel,
        Err(_) => panic!("ahhhh"),
    };
}

pub fn prompt_options(message: &str, options: Vec<&'static str>) -> Vec<&'static str>{
    return match MultiSelect::new(message, options).prompt() {
        Ok(sels) => sels,
        Err(_) => panic!("oh no!"),
    };
}