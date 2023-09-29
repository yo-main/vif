use crate::cli;
use std::fs::canonicalize;
use std::path::PathBuf;

pub struct Config {
    pub entrypoint: PathBuf,
}

pub fn get_config() -> Config {
    let cli = cli::get_cli();
    let matches = cli.get_matches();

    let entrypoint = canonicalize(matches.get_one::<String>("entrypoint").unwrap()).unwrap();

    return Config { entrypoint };
}
