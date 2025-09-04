use crate::prelude::*;

const GLOBAL_AUTH_OPENAI_NAME: &str = "openai";

const GLOBAL_AUTH_OPENAI_API_TOKEN_NAME: &str = "api_token";

// TODO: Consider using admin tokens for projects management
// const GLOBAL_AUTH_OPENAI_ADMIN_TOKEN_NAME: &str = "admin_token";

#[derive(Error, Debug)]
pub enum OpenAiAuthError {
    #[error(transparent)]
    Global(#[from] GlobalError),

    #[error("Failed to update OpenAI auth token: {0}")]
    Update(anyhow::Error),
}

pub struct OpenAiAuth {
    token: OpenAiToken,
}

impl OpenAiAuth {
    pub fn new(token: OpenAiToken) -> Self {
        Self { token }
    }

    pub fn detect(global: &Global) -> Option<Self> {
        global
            .auth
            .get(GLOBAL_AUTH_OPENAI_NAME)
            .and_then(|table| table.get(GLOBAL_AUTH_OPENAI_API_TOKEN_NAME))
            .and_then(|item| item.as_str())
            .and_then(|token| Some(Self::new(token.into())))
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn persist(&self, global: &mut Global) -> Result<(), OpenAiAuthError> {
        global
            .auth
            .update(GLOBAL_AUTH_OPENAI_NAME, |table: &mut Table| {
                table[GLOBAL_AUTH_OPENAI_API_TOKEN_NAME] = value(self.token.as_str());
                Ok(())
            })
            .map_err(|err| OpenAiAuthError::Update(err))?;
        global.persist_auth()?;
        Ok(())
    }
}
