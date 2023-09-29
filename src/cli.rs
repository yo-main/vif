use clap::Arg;
use clap::Command;

pub fn get_cli() -> Command {
    Command::new("zeus").arg(Arg::new("entrypoint").required(true))
}
