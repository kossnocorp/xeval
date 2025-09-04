use crate::prelude::*;

#[derive(clap::Args)]
pub struct InitArgs {
    /// Optional path to initialize the project in
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Force initialization. Overwrite existing files.
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("Failed to read input from terminal")]
    Terminal(#[from] dialoguer::Error),
}

pub struct InitCmd {}

impl InitCmd {
    pub fn run<'a>(_cli: &'a Cli, args: &'a InitArgs) -> Result<(), InitError> {
        println!("INIT with path={:?}, force={:?}", args.path, args.force);
        let _config = Config::init(&args.path, args.force)?;

        let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Login with OpenAI to use their engine?")
            .interact()?;

        println!("CONFIRM: {}", confirmation);

        Ok(())
    }
}
