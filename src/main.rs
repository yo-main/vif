#![feature(map_try_insert)]

mod application;
mod ast;
mod ast_printer;
mod builtin;
mod cli;
mod config;
mod environment;
mod errors;
mod interpreter;
mod parser;
mod tokenizer;
mod tokens;
mod visitor;
mod zeus_function;

fn main() {
    let config = config::get_config();

    let mut zeus = application::Zeus::init();

    match config.entrypoint {
        Some(path) => zeus.run_file(path),
        _ => zeus.run_prompt(),
    }
    .unwrap();
}
