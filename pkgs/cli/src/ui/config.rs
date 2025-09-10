use crate::prelude::*;

pub struct UiConfig {}

impl UiConfig {
    pub fn inquire_evals_glob() -> Result<String> {
        let evals_glob = Input::with_theme(UiTheme::for_dialoguer())
            .with_prompt("Eval files pattern")
            .default(Config::default_evals_glob())
            .interact_text()?;
        Ok(evals_glob)
    }
}
