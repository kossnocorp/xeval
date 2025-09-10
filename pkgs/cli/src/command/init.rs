use crate::prelude::*;

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("Failed to read input from terminal")]
    Terminal(#[from] dialoguer::Error),

    #[error(transparent)]
    Global(#[from] GlobalError),

    #[error(transparent)]
    OpenAi(#[from] OpenAiError),

    #[error(transparent)]
    OpenAiAuth(#[from] OpenAiAuthError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(clap::Args)]
pub struct InitArgs {
    /// Optional path to initialize the project in
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Force initialization. Overwrite existing files.
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

pub struct InitCmd {}

impl InitCmd {
    pub async fn run<'a>(cli: &'a Cli, args: &'a InitArgs) -> Result<(), InitError> {
        let mut global = Global::resolve()?;
        let _ = Auth::ensure(&mut global, AuthState::New).await?;

        let evals_glob = UiConfig::inquire_evals_glob()?;

        let mut config = Config::init(&args.path, args.force)?;
        config.evals = evals_glob;

        let config_path = cli.config.clone();
        let project = Project {};

        Config::write_new(&args.path, args.force, &config)?;

        Ok(())
    }
}
