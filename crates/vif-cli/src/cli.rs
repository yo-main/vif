use clap::Arg;
use clap::Command;

pub fn get_cli() -> Command {
    Command::new("vif").arg(Arg::new("entrypoint").required(false))
}