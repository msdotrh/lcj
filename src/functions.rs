use colored::*;

pub fn help() {
    let help_dialog = format!(
        "{}
        lcj init {} {}
            (Initalize a testcase with the name {}; takes input and output from {})
        lcj run  {}
            (Run testcase)
        lcj list
            (List testcases)
        ",
        "Usage:".bright_blue().bold(),
        "<testcase-name>".yellow(),
        "<(testcase-input,output) dir>".green(),
        "\"testcase-name\"".yellow(),
        "\"dir\"".green(),
        "<testcase-name>".yellow()
    );
    println!("{}", help_dialog);
}

pub fn run() {
    todo!();
}

pub fn list() {
    todo!();
}

pub fn init() {
    todo!();
}

pub fn invaild() {
    println!("{}", "Invaild command".red().bold());
    help();
}

pub fn delete() {
    todo!();
}
