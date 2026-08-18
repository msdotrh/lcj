use colored::*;

use crate::testcases;

fn write_to_toml(table: &testcases::TestCasesVector) {}

pub fn help() {
    let help_dialog = format!(
        "{}
        lcj init {} {} {}
            (Initalize a testcase with the name {}; takes input and compares output from {}; use {})
        lcj run  {}
            (Run testcase)
        lcj list
            (List testcases)
        ",
        "Usage:".bright_blue().bold(),
        "<testcase-name>".yellow(),
        "<binary>".bright_purple(),
        "<(testcase-input,output) dir>".green(),
        "\"testcase-name\"".yellow(),
        "\"dir\"".green(),
        "\"binary\"".bright_purple(),
        "<testcase-name>".yellow()
    );
    println!("{}", help_dialog);
}

pub fn run() {
    todo!();
}

pub fn list(table: &testcases::TestCasesVector) {
    dbg!(table);
}

pub fn init(table: &mut testcases::TestCasesVector, argv: &Vec<String>) {}

pub fn invaild() {
    println!("{}", "Invaild command".red().bold());
    help();
}

pub fn delete(table: &mut testcases::TestCasesVector, argv: &Vec<String>) {
    if argv.len() < 3 {
        println!("{} is missing!", "<testcase-name>".yellow());
    }

    let name = argv[2].clone();
    let prev_size = table.vector.len();
    table.vector.retain(|x| *x.name != name);
    if prev_size == table.vector.len() {
        println!("\"{}\" is not found!, deleted nothing", name.red());
    }

    write_to_toml(table);
}

pub fn reset() {
    std::fs::write("testcases.toml", "").expect("Failed to clear testcases.toml");
}
