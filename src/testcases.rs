use std::ops::Not;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct TestCase {
    pub name: String,
    pub iodir: String,
    pub binpath: String,
    pub time_limit: u32,
    pub memory_limit: u32,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
pub struct TestCasesVector {
    #[serde(rename = "testcase", default)]
    pub vector: Vec<TestCase>,
}

pub fn load_test_cases() -> TestCasesVector {
    if std::fs::exists("testcases.toml").unwrap().not() {
        std::fs::write("testcases.toml", "").expect("Can't write to testcase.toml");
    }

    let contents = std::fs::read_to_string("testcases.toml").expect("Failed to read file");

    let table =
        toml::from_str::<TestCasesVector>(contents.as_str()).expect("Failed to parse .toml");
    table
}
