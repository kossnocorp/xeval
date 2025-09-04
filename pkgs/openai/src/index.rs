use crate::prelude::*;

#[derive(Error, Debug)]
pub enum OpenAiError {
    #[error("Failed to perform verification request: {0}")]
    VerificationRequest(reqwest::Error),

    #[error("Failed to verify token (server responded with {0} {1})")]
    InvalidToken(String, String),

    #[error("Failed to perform projects request: {0}")]
    ProjectsRequest(reqwest::Error),

    #[error("Failed to parse projects response: {0}")]
    ProjectsDeserialize(reqwest::Error),
}

pub struct OpenAi {
    pub auth: OpenAiAuth,
}

impl OpenAi {
    pub async fn new(auth: OpenAiAuth) -> Result<Option<Self>, OpenAiError> {
        let openai = Self { auth };
        let verification = openai.verify().await?;
        match verification {
            OpenAiTokenVerification::Valid => Ok(Some(openai)),
            OpenAiTokenVerification::Invalid {
                response: _response,
            } => Ok(None),
        }
    }

    pub async fn from_token(token: OpenAiToken) -> Result<Option<Self>, OpenAiError> {
        let auth = OpenAiAuth::new(token);
        Self::new(auth).await
    }

    pub async fn detect(global: &Global) -> Result<Option<Self>, OpenAiError> {
        let auth = OpenAiAuth::detect(global);

        if let Some(auth) = auth {
            Self::new(auth).await
        } else {
            Ok(None)
        }
    }

    pub async fn verify(&self) -> Result<OpenAiTokenVerification, OpenAiError> {
        let client = Client::new();
        let response = client
            .get("https://api.openai.com/v1/models")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.auth.token()),
            )
            .send()
            .await
            .map_err(|err| OpenAiError::VerificationRequest(err))?;

        if response.status().is_success() {
            Ok(OpenAiTokenVerification::Valid)
        } else {
            Ok(OpenAiTokenVerification::Invalid { response })
        }
    }
}

pub enum OpenAiTokenVerification {
    Valid,
    Invalid { response: Response },
}
