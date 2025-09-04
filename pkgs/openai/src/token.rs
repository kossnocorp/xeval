pub struct OpenAiToken(String);

impl OpenAiToken {
    pub fn new(token: String) -> Self {
        OpenAiToken(token)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for OpenAiToken {
    fn from(token: String) -> Self {
        OpenAiToken::new(token)
    }
}

impl From<&str> for OpenAiToken {
    fn from(token: &str) -> Self {
        OpenAiToken::new(token.into())
    }
}
