mod functions;
mod parser;
mod types;

use crate::types::testcases;
use std::env;

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let mut table: testcases::TestCasesVector = testcases::load_test_cases();
    parser::arguments_handler(&argv, &mut table);
}
