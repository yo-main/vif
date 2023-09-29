mod config;
mod cli;

fn main() {
    let config = config::get_config();

    println!("{:?}", config.entrypoint);
}
