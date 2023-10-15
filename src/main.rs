mod application;
mod ast;
mod ast_printer;
mod cli;
mod config;
mod errors;
mod interpreter;
mod parser;
mod tokenizer;
mod tokens;
mod visitor;

fn main() {
    let config = config::get_config();

    let zeus = application::Zeus::init();

    match config.entrypoint {
        Some(path) => zeus.run_file(path),
        _ => zeus.run_prompt(),
    }
    .unwrap();
}
