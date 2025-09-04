use crate::prelude::*;

#[derive(Error, Debug)]
pub enum OpenAiError {
    #[error("Failed to perform verification request ({0})")]
    VerificationRequest(reqwest::Error),

    #[error("Failed to verify token (server responded with {0} {1})")]
    InvalidToken(String, String),
}

pub struct OpenAi {
    pub auth: OpenAiAuth,
}

impl OpenAi {
    pub async fn new(auth: OpenAiAuth) -> Result<Self, OpenAiError> {
        let openai = Self { auth };
        openai.verify().await.map(|_| openai)
    }

    pub async fn from_token(token: String) -> Result<Self, OpenAiError> {
        let auth = OpenAiAuth::new(token);
        Self::new(auth).await
    }

    pub async fn detect(global: &Global) -> Result<Option<Self>, OpenAiError> {
        let auth = OpenAiAuth::detect(global);

        if let Some(auth) = auth {
            let openai = Self::new(auth).await?;
            Ok(Some(openai))
        } else {
            Ok(None)
        }
    }

    pub async fn verify(&self) -> Result<(), OpenAiError> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.openai.com/v1/models")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.auth.token()),
            )
            .send()
            .await
            .map_err(|err| OpenAiError::VerificationRequest(err))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(OpenAiError::InvalidToken(
                res.status().to_string(),
                res.text().await.unwrap_or_default(),
            ))
        }
    }
}
