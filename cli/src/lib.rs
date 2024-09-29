use clap::{CommandFactory, Parser, Subcommand};
use subcommands::{Challenge, Disprove, Register, Signer, Tx};

pub mod subcommands;

#[derive(Debug, Parser)]
#[clap(author, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Subcommands>,
    #[clap(long = "version", short = 'V', help = "Print version info and exit")]
    pub version: bool,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    #[clap(about = "Signer management commands")]
    Signer(Signer),
    #[clap(about = "Challenge a proof")]
    Challenge(Challenge),
    #[clap(about = "Transaction commands")]
    Tx(Tx),
    #[clap(about = "Disprove commands")]
    Disprove(Disprove),
    #[clap(about = "Register commands")]
    Register(Register),
}

pub async fn run_command(cli: Cli) -> anyhow::Result<()> {
    match (cli.version, cli.command) {
        (false, None) => Ok(Cli::command().print_help()?),
        (true, _) => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        (false, Some(command)) => match command {
            Subcommands::Signer(cmd) => cmd.run().await,
            Subcommands::Challenge(cmd) => cmd.run().await,
            Subcommands::Tx(cmd) => cmd.run().await,
            Subcommands::Disprove(cmd) => cmd.run().await,
            Subcommands::Register(cmd) => cmd.run().await,
        },
    }
}
