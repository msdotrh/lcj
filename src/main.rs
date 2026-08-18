mod functions;
mod messages;
mod parser;
use std::env;

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    parser::arguments_handler(&argv);
}
