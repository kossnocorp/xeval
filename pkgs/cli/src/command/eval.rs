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

    #[error(transparent)]
    Global(#[from] GlobalError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub struct EvalCmd {}

impl EvalCmd {
    pub async fn run<'a>(cli: &'a Cli, args: &'a EvalArgs) -> Result<(), EvalError> {
        println!("EVAL with watch={:?}", args.watch);

        let config = Config::find(&cli.config)?;
        let mut global = Global::resolve()?;
        let auth = Auth::ensure(&mut global, AuthState::Existing).await?;

        println!("CONFIG {:?}", config);
        Ok(())
    }
}
