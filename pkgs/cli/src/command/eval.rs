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
    OpenAiEvals(#[from] OpenAiEvalsError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub struct EvalCmd {}

impl EvalCmd {
    pub async fn run<'a>(cli: &'a Cli, args: &'a EvalArgs) -> Result<(), EvalError> {
        let mut global = Global::resolve()?;
        let auth = Auth::ensure(&mut global, AuthState::Existing).await?;

        let spinner = UiTheme::start_spinner("Syncing OpenAI evals");
        let project = Project {
            path: std::env::current_dir().context("Failed to get current directory")?,
        };
        let state = OpenAiLocalProjectState::new(&project)?;
        let _evals = OpenAiLocalEvals::sync(&auth, &state, false, None).await?;
        spinner.finish_and_clear();

        Ok(())
    }
}
