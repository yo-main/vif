use clap::Arg;
use clap::Command;

pub fn get_cli() -> Command {
    Command::new("vif")
        .arg(
            Arg::new("assembly")
                .long("assembly")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(Arg::new("ast").long("ast").action(clap::ArgAction::SetTrue))
        .arg(Arg::new("entrypoint").required(false))
}
