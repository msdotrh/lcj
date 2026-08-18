use crate::{functions, messages::Message};

fn parse_arguments_into_messages(argv: &Vec<String>) -> Message {
    // <exe>
    if argv.len() == 1 {
        return Message::Help;
    }

    // handle number of words
    let 2..=4 = argv.len() else {
        return Message::Invalid;
    };

    // <exe> ... (more than 2 words)
    let out_message = match argv[1].as_str() {
        "help" | "h" => Message::Help,
        "init" | "i" | "add" | "a" => Message::Init,
        "run" | "r" => Message::Run,
        "list" | "ls" | "l" => Message::List,
        "delete" | "del" | "d" => Message::Delete,
        _ => Message::Invalid,
    };
    out_message
}

pub fn arguments_handler(argv: &Vec<String>) {
    let message = parse_arguments_into_messages(argv);
    match message {
        Message::Run => functions::run(),
        Message::List => functions::list(),
        Message::Help => functions::help(),
        Message::Invalid => functions::invaild(),
        Message::Init => functions::init(),
        Message::Delete => functions::delete(),
    }
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn print_arguments(argv: Vec<String>) {
    dbg!(argv);
}
