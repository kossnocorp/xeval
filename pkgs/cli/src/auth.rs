use crate::prelude::*;

pub struct Auth {
    pub openai: OpenAi,
}

pub enum AuthState {
    New,
    Existing,
}

impl Auth {
    pub async fn ensure(global: &mut Global, state: AuthState) -> Result<Self> {
        let openai = match OpenAi::detect(&global).await? {
            Some(openai) => {
                if matches!(state, AuthState::New) {
                    UiMessage::info("Using OpenAI API token stored in state");
                }

                openai
            }
            None => loop {
                let token = UiOpenAiToken::inquire_token()?;

                match token {
                    Some(token) => {
                        let openai = OpenAi::from_token(token).await?;
                        match openai {
                            Some(openai) => {
                                openai.auth.persist(global)?;
                                UiMessage::success("OpenAI API token verified & saved");
                                break openai;
                            }

                            None => {
                                UiMessage::warn("Token is invalid!");
                                match UiOpenAiToken::inquire_retry()? {
                                    true => continue,
                                    false => {}
                                }
                            }
                        }
                    }

                    None => {}
                };

                UiMessage::warn("No token provided");
                exit(1);
            },
        };

        Ok(Self { openai })
    }
}
