mod application;
mod cli;
mod config;
mod errors;

fn main() {
    let config = config::get_config();

    let zeus = application::Zeus::init();

    match config.entrypoint {
        Some(path) => zeus.run_file(path),
        _ => zeus.run_prompt(),
    }
    .unwrap();
}
