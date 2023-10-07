mod application;
mod cli;
mod config;
mod errors;
mod tokenizer;
mod tokens;

fn main() {
    let config = config::get_config();

    let zeus = application::Zeus::init();

    match config.entrypoint {
        Some(path) => zeus.run_file(path),
        _ => zeus.run_prompt(),
    }
    .unwrap();
}
