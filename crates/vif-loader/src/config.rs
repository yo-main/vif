use crate::cli;
use lazy_static::lazy_static;
use std::fs::canonicalize;
use std::path::PathBuf;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

pub enum Print {
    Assembly(PathBuf),
    Ast(PathBuf),
}

pub enum Action {
    Build(PathBuf),
    Execute(PathBuf),
    ExecuteFromStdin,
    Print(Print),
}

pub struct Config {
    pub debug: bool,
    pub log_level: log::LevelFilter,
    pub action: Action,
}

pub fn get_config() -> Config {
    let cli = cli::get_cli();
    let matches = cli.get_matches();

    let action = match matches.subcommand() {
        Some(("run", subcommant_matches)) => Action::Execute(PathBuf::from(
            subcommant_matches.get_one::<String>("entrypoint").unwrap(),
        )),
        Some(("build", subcommand_matches)) => Action::Build(PathBuf::from(
            subcommand_matches.get_one::<String>("entrypoint").unwrap(),
        )),
        Some(("print", subcommand_matches)) => {
            let path = PathBuf::from(subcommand_matches.get_one::<String>("entrypoint").unwrap());
            if subcommand_matches.get_flag("assembly") {
                Action::Print(Print::Assembly(path))
            } else if subcommand_matches.get_flag("ast") {
                Action::Print(Print::Ast(path))
            } else {
                Action::Print(Print::Assembly(path))
            }
        }
        Some(("compile", _)) => Action::ExecuteFromStdin,
        _ => unreachable!(),
    };

    let debug = std::env::var("DEBUG").and(Ok(true)).unwrap_or(false);
    let log_level = std::env::var("VIF_LOG_LEVEL")
        .map(|lvl| lvl.parse().unwrap())
        .unwrap_or(if debug {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Error
        });

    return Config {
        debug,
        log_level,
        action,
    };
}
