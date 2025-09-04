use crate::prelude::*;

pub struct UiOpenAiToken {}

impl UiOpenAiToken {
    pub fn inquire_token() -> Result<Option<OpenAiToken>> {
        let token = Password::with_theme(UiTheme::for_dialoguer())
            .with_prompt("Enter your OpenAI API token")
            .allow_empty_password(false)
            .interact()?;
        Ok(Some(token.into()))
    }

    pub fn inquire_retry() -> Result<bool> {
        let should_retry = Confirm::with_theme(UiTheme::for_dialoguer())
            .with_prompt("Do you want to try again?")
            .interact()?;
        Ok(should_retry)
    }
}
