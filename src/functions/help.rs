use colored::*;

pub fn help() {
    let help_dialog = format!(
        "{}
        lcj init {} {} {}
            (Initalize a testcase with the name {}; takes input and compares output from {}; use {})
        lcj run  {}
            (Run testcase)
        lcj list
            (List testcases)
        lcj reset
            (Reset, clear all testcases)
        lcj delete {}
            (Delete testcase)
        ",
        "Usage:".bright_blue().bold(),
        "<testcase-name>".yellow(),
        "<binary>".bright_purple(),
        "<(testcase-input,output) dir>".green(),
        "\"testcase-name\"".yellow(),
        "\"dir\"".green(),
        "\"binary\"".bright_purple(),
        "<testcase-name>".yellow(),
        "\"testcase-name\"".yellow(),
    );
    println!("{}", help_dialog);
}
