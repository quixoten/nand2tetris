use std::env;
use std::path::PathBuf;

mod emitter;
mod lexer;
mod parser;
mod tokens;

use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <source.vm>", args[0]);
    }

    let source_path = PathBuf::from(&args[1]);
    Parser::parse(source_path);
}
