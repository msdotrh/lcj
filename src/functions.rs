use crate::testcases::{self};
use colored::*;
use std::{
    collections::{BTreeMap, HashMap}, fs::{self, File}, io::Read, ops::Not, path::{Path, PathBuf, absolute}, process::{Command, Stdio}, thread, time::{Duration, Instant},
};

#[derive(Debug, Clone)]
struct IOTestCase {
    inp: Option<PathBuf>,
    out: Option<PathBuf>,
}

#[derive(PartialEq, Debug)]
enum TestCaseResult {
    AC,
    WA,
    TLE,
    MLE,
    RE,
}

impl IOTestCase {
    pub fn new(inp: Option<PathBuf>, out: Option<PathBuf>) -> Self {
        Self { inp: inp, out: out }
    }
}

fn compare_tokens(output: &String, expected_output: &PathBuf) -> TestCaseResult {
    let answer = fs::read_to_string(expected_output).expect("Cannot read the expected output!");
    let answer_token = answer.split_whitespace().collect::<Vec<&str>>();
    let output_token = output.split_whitespace().collect::<Vec<&str>>();
    match answer_token == output_token {
        true => TestCaseResult::AC,
        false => TestCaseResult::WA,
    }
}

fn execute_program(
    binary_path: &PathBuf,
    input: &PathBuf,
    expected_output: &PathBuf,
) -> TestCaseResult {
    let input_file = File::open(input).expect("Cannot open input file");
    let mut command = Command::new(binary_path);
    command
        .stdin(Stdio::from(input_file))
        .stdout(Stdio::piped());

    let mut child = command.spawn().expect("Cannot spawn program");
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = thread::spawn(move || {
        let mut output = Vec::new();
        let mut reader = stdout;
        reader
            .read_to_end(&mut output)
            .expect("Failed to read stdout");
        output
    });

    let start = Instant::now();

    loop {
        match child.try_wait().expect("Failed to poll child") {
            Some(status) => {
                let output = reader.join().expect("Failed to join stdout reader");
                let output_string = String::from(String::from_utf8_lossy(&output));

                if !status.success() {
                    return TestCaseResult::RE;
                }

                return compare_tokens(&output_string, expected_output);
            }

            None if start.elapsed() >= Duration::from_millis(1500) => {
                child.kill().expect("Failed to kill process");
                let _ = child.wait();
                let _ = reader.join();
                return TestCaseResult::TLE;
            }

            None => {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

fn write_to_toml(table: &testcases::TestCasesVector) {
    let toml_string = toml::to_string_pretty(table).expect("Can not convert TOML file to a string");

    std::fs::write("testcases.toml", toml_string).expect("Can not write to TOML file");
}

fn pairing(io_directory: &Path) -> BTreeMap<String, IOTestCase> {
    let paths = fs::read_dir(io_directory);
    let mut pairs: BTreeMap<String, IOTestCase> = BTreeMap::new();
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
    let filtered_pairs: BTreeMap<_, _> = pairs
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
                "Cannot find testcase named {}
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

    let mut results: Vec<TestCaseResult> = Vec::new();
    for (case, iocase) in pairs {
        let result = execute_program(
            &binary_path.to_path_buf(),
            &iocase
                .inp
                .expect(format!("Cannot open input of IOCASE {}", case.yellow().bold()).as_str()),
            &iocase
                .out
                .expect(format!("Cannot open output of IOCASE {}", case.yellow().bold()).as_str()),
        );
        println!("{} {}: {}", "CASE", case, match result {
            TestCaseResult::WA => format!("{:#?}", result).red().bold(),
            TestCaseResult::AC =>  format!("{:#?}", result).green().bold(),
            TestCaseResult::TLE =>  format!("{:#?}", result).black().on_white().bold(),
            TestCaseResult::MLE =>  format!("{:#?}", result).bright_yellow().bold(),
            TestCaseResult::RE =>  format!("{:#?}", result).white().on_red().bold(),
        });
        results.push(result);
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
