use crate::testcases;
use colored::*;
use std::{
    collections::HashMap,
    fs::{self, File},
    ops::Not,
    path::{Path, PathBuf, absolute},
    process::{Command, Stdio},
};

#[derive(Debug, Clone)]
struct IOTestCase {
    inp: Option<PathBuf>,
    out: Option<PathBuf>,
}

impl IOTestCase {
    pub fn new(inp: Option<PathBuf>, out: Option<PathBuf>) -> Self {
        Self { inp: inp, out: out }
    }
}

fn execute_program(binary_path: &PathBuf, input: &PathBuf)-> String {
    let input_file = File::open(input).expect("Cannot open input file");
    let execute = Command::new(binary_path)
        .stdin(Stdio::from(input_file))
        .stdout(Stdio::piped())
        .output()
        .expect(
            format!(
                "Cannot execute {} successfully",
                binary_path.to_string_lossy().red().bold(),
            )
            .as_str(),
        );

    let output_buffer = String::from_utf8(execute.stdout).expect("Cannot read output_buffer");
    println!("{}", output_buffer);
    output_buffer
}

fn write_to_toml(table: &testcases::TestCasesVector) {
    let toml_string = toml::to_string_pretty(table).expect("Can not convert TOML file to a string");

    std::fs::write("testcases.toml", toml_string).expect("Can not write to TOML file");
}

fn pairing(io_directory: &Path) -> HashMap<String, IOTestCase> {
    let paths = fs::read_dir(io_directory);
    let mut pairs: HashMap<String, IOTestCase> = HashMap::new();
    for e in paths.unwrap() {
        let e = e.unwrap();
        let file_path = e.path();
        let file_name = file_path.file_stem().unwrap().to_str().unwrap().to_string();
        let Some(file_extension) = file_path.extension() else {
            continue;
        };
        let file_extesion_string = file_extension.to_string_lossy().into_owned();
        /*
        <check if file_name in hash>
        if yes:
            if .out => ::new(hash[file_name].inp, file_name.out);
            if .inp => ::new(file_name..inp, hash[file_name].out);
         */
        if let Some(test_case) = pairs.get(&file_name) {
            let inp = test_case.inp.clone();
            let out = test_case.out.clone();

            let new_test_case = match file_extesion_string.as_str() {
                "inp" => IOTestCase::new(Some(file_path), out),
                "out" => IOTestCase::new(inp, Some(file_path)),
                _ => continue,
            };
            pairs.insert(file_name, new_test_case);
        } else {
            pairs.insert(
                file_name,
                match file_extesion_string.as_str() {
                    "inp" => IOTestCase::new(Some(file_path), None),
                    "out" => IOTestCase::new(None, Some(file_path)),
                    _ => continue,
                },
            );
        }
    }
    let filtered_pairs: HashMap<_, _> = pairs
        .iter()
        .filter(|&(_file_name, case)| case.inp.is_some() && case.out.is_some())
        .map(|(name, case)| (name.clone(), case.clone()))
        .collect();
    if filtered_pairs.len() < pairs.len() {
        println!("Some cases don't have a .inp, or an .out file, consider adding");
    }
    filtered_pairs
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

pub fn run(table: &testcases::TestCasesVector, argv: &Vec<String>) {
    // identify
    let name = argv[2].clone();
    let find_testcase = table.vector.iter().find(|x| x.name == name);
    let testcase_wrapped =
        find_testcase.ok_or(format!("Cannot find testcase named {}", name.red().bold()));
    match testcase_wrapped {
        Ok(_) => {}
        Err(_) => {
            println!(
                "
Cannot find testcase named {}
Consider list testcases with {}",
                name.red().bold(),
                "lcj list".yellow().bold(),
            );
            std::process::exit(1);
        }
    }

    let testcase = testcase_wrapped.unwrap();

    // access Path
    let io_directory = Path::new(&testcase.iodir);
    let binary_path = Path::new(&testcase.binpath);

    // check if Path is valid
    if binary_path.is_file().not() {
        println!(
            "{} does not exist",
            binary_path.display().to_string().yellow().bold()
        );
        std::process::exit(0);
    }
    if io_directory.is_dir().not() {
        println!(
            "{} does not exist",
            binary_path.display().to_string().yellow().bold()
        );
        std::process::exit(0);
    }

    // Pairing
    let pairs = pairing(io_directory);

    for (case, iocase) in pairs {
        // Execute program and compare with inp, out
        let out = execute_program(&binary_path.to_path_buf(), &iocase.inp.unwrap());

    }
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
        memory_limit: 100,
    };

    dbg!(&new_case.clone());
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
    println!("Cleared testcase.toml");
}
