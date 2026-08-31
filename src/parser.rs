use crate::{functions, testcases, types::messages::Message};

fn parse_arguments_into_messages(argv: &Vec<String>) -> Message {
    // <exe>
    if argv.len() == 1 {
        return Message::Help;
    }

    // handle number of words
    let 2..=5 = argv.len() else {
        return Message::Invalid;
    };

    // <exe> ... (more than 2 words)
    let out_message = match argv[1].as_str() {
        "help" | "h" => Message::Help,
        "init" | "i" | "add" | "a" => Message::Init,
        "run" | "r" => Message::Run,
        "list" | "ls" | "l" => Message::List,
        "delete" | "del" | "d" => Message::Delete,
        "debug" => Message::Debug,
        "reset" => Message::Reset,
        _ => Message::Invalid,
    };
    out_message
}

pub fn arguments_handler(argv: &Vec<String>, table: &mut testcases::TestCasesVector) {
    let message = parse_arguments_into_messages(argv);
    match message {
        Message::Run => functions::execute::run(table, argv),
        Message::List => functions::file_handler::list(table),
        Message::Help => functions::help::help(),
        Message::Invalid => functions::invalid::invaild(),
        Message::Init => functions::file_handler::init(table, argv),
        Message::Delete => functions::file_handler::delete(table, argv),
        Message::Reset => functions::file_handler::reset(),
        Message::Debug => {}
    }
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn print_arguments(argv: Vec<String>) {
    dbg!(argv);
}
