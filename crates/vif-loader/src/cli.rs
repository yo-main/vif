use clap::Arg;
use clap::Command;

pub fn get_cli() -> Command {
    Command::new("vif")
        .subcommand(Command::new("run").arg(Arg::new("entrypoint").required(true)))
        .subcommand(Command::new("build").arg(Arg::new("entrypoint").required(true)))
        .subcommand(Command::new("compile"))
        .subcommand(
            Command::new("print")
                .arg(
                    Arg::new("assembly")
                        .long("assembly")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(Arg::new("ast").long("ast").action(clap::ArgAction::SetTrue))
                .arg(Arg::new("entrypoint").required(true)),
        )
}
