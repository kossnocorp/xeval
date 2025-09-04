use crate::prelude::*;

#[derive(clap::Args)]
pub struct EvalArgs {
    /// Watch for changes.
    #[arg(short, long, default_value_t = false)]
    watch: bool,
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error(transparent)]
    Config(#[from] ConfigError),
}

pub struct EvalCmd {}

impl EvalCmd {
    pub fn run<'a>(cli: &'a Cli, args: &'a EvalArgs) -> Result<(), EvalError> {
        println!("EVAL with watch={:?}", args.watch);
        let config = Config::find(&cli.config)?;
        println!("CONFIG {:?}", config);
        Ok(())
    }
}
