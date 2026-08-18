use crate::{functions, messages::Message, testcases};

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
        _ => Message::Invalid,
    };
    out_message
}

pub fn arguments_handler(argv: &Vec<String>, table: &mut testcases::TestCasesVector) {
    let message = parse_arguments_into_messages(argv);
    match message {
        Message::Run => functions::run(),
        Message::List => functions::list(table),
        Message::Help => functions::help(),
        Message::Invalid => functions::invaild(),
        Message::Init => functions::init(table, argv),
        Message::Delete => functions::delete(table, argv),
        Message::Reset => functions::reset(),
        Message::Debug => {}
    }
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn print_arguments(argv: Vec<String>) {
    dbg!(argv);
}
