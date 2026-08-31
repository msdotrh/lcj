use crate::functions::help;
use colored::*;

pub fn invaild() {
    println!("{}", "Invaild command".red().bold());
    help::help();
}
