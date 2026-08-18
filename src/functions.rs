use colored::*;

use crate::testcases;

fn write_to_toml(table: &testcases::TestCasesVector) {
    let toml_string = toml::to_string_pretty(table).expect("Can not convert TOML file to a string");

    std::fs::write("testcases.toml", toml_string).expect("Can not write to TOML file");
}

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

pub fn run() {
    todo!();
}

pub fn list(table: &testcases::TestCasesVector) {
    dbg!(table);
}

pub fn init(table: &mut testcases::TestCasesVector, argv: &Vec<String>) {
    if argv.len() < 5 {
        println!(
            "Not enough arguments! Currently having {} arguments",
            argv.len()
        );
        return;
    }

    let testcase_name = argv[2].clone();
    let binary_path = argv[3].clone();
    let io_directory = argv[4].clone();

    if table.vector.iter().any(|x| x.name == testcase_name) {
        println!("{} exists", testcase_name.red());
        return;
    }

    let new_case = testcases::TestCase {
        name: testcase_name,
        iodir: io_directory,
        binpath: binary_path,
        time_limit: 1000,
        memory_limit: 100,
    };

    table.vector.push(new_case);

    write_to_toml(table);
}

pub fn invaild() {
    println!("{}", "Invaild command".red().bold());
    help();
}

pub fn delete(table: &mut testcases::TestCasesVector, argv: &Vec<String>) {
    if argv.len() < 3 {
        println!("{} is missing!", "<testcase-name>".yellow());
        return;
    }

    let name = argv[2].clone();
    let prev_size = table.vector.len();
    table.vector.retain(|x| *x.name != name);
    if prev_size == table.vector.len() {
        println!("\"{}\" is not found!, deleted nothing", name.red());
        return;
    }

    write_to_toml(table);
}

pub fn reset() {
    std::fs::write("testcases.toml", "").expect("Failed to clear testcases.toml");
}
