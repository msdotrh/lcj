use crate::testcases;
use colored::*;
use std::path::absolute;

fn write_to_toml(table: &testcases::TestCasesVector) {
    let toml_string = toml::to_string_pretty(table).expect("Can not convert TOML file to a string");

    std::fs::write("testcases.toml", toml_string).expect("Can not write to TOML file");
}

pub fn list(table: &testcases::TestCasesVector) {
    dbg!(table);
}

pub fn init(table: &mut testcases::TestCasesVector, argv: &[String]) {
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

    dbg!(&io_directory);

    if table.vector.iter().any(|x| x.name == testcase_name) {
        println!("{} exists", testcase_name.red());
        return;
    }

    let new_case = testcases::TestCase {
        name: testcase_name,
        iodir: absolute(io_directory).unwrap().display().to_string(),
        binpath: absolute(binary_path).unwrap().display().to_string(),
        time_limit: 1000,
        memory_limit: 256,
    };

    dbg!(&new_case.clone());
    table.vector.push(new_case);

    write_to_toml(table);
}

pub fn delete(table: &mut testcases::TestCasesVector, argv: &[String]) {
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
    println!("Cleared testcase.toml");
}
