use crate::prelude::*;

const GLOBAL_AUTH_OPENAI_NAME: &str = "openai";

#[derive(Error, Debug)]
pub enum OpenAiAuthError {
    #[error(transparent)]
    Global(#[from] GlobalError),

    #[error("Failed to update OpenAI auth token: {0}")]
    Update(anyhow::Error),
}

pub struct OpenAiAuth {
    token: String,
}

impl OpenAiAuth {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn detect(global: &Global) -> Option<Self> {
        global
            .auth
            .get(GLOBAL_AUTH_OPENAI_NAME)
            .and_then(|table| table["token"].as_str())
            .and_then(|token| Some(Self::new(token.into())))
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn persist(&self, global: &mut Global) -> Result<(), OpenAiAuthError> {
        global
            .auth
            .update(GLOBAL_AUTH_OPENAI_NAME, |table: &mut Table| {
                table["token"] = value(self.token.clone());
                Ok(())
            })
            .map_err(|err| OpenAiAuthError::Update(err))?;
        global.persist_auth()?;
        Ok(())
    }
}
