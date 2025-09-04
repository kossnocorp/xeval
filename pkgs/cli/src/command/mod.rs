use crate::prelude::*;

mod eval;
pub use eval::*;

mod init;
pub use init::*;

#[derive(clap::Subcommand)]
pub enum Command {
    /// Initialize a new xeval project in an existing directory
    Init(InitArgs),

    /// Run all evals in the project.
    Eval(EvalArgs),
}

impl Command {
    pub async fn run(cli: &Cli) -> Result<()> {
        match &cli.command {
            Some(Command::Init(args)) => Ok(InitCmd::run(cli, args).await?),

            Some(Command::Eval(args)) => Ok(EvalCmd::run(cli, args).await?),

            None => unreachable!("No command was provided"),
        }
    }
}
