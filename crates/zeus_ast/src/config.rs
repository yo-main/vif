use crate::cli;
use std::fs::canonicalize;
use std::path::PathBuf;

pub struct Config {
    pub entrypoint: Option<PathBuf>,
}

pub fn get_config() -> Config {
    let cli = cli::get_cli();
    let matches = cli.get_matches();

    let entrypoint = matches
        .get_one::<String>("entrypoint")
        .and_then(|path| Some(canonicalize(path).expect("Could not parse the provided path")));

    return Config { entrypoint };
}
