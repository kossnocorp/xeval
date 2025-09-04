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
    pub async fn run<'a>(_cli: &'a Cli, args: &'a InitArgs) -> Result<(), InitError> {
        println!("INIT with path={:?}, force={:?}", args.path, args.force);

        let mut global = Global::resolve()?;
        let auth = Auth::ensure(&mut global, AuthState::New).await?;
        let _ = OpenAiLocalProject::select(&auth).await?;

        let _ = Config::init(&args.path, args.force)?;

        Ok(())
    }
}
