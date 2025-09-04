use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiResponseList<T> {
    pub object: String,
    pub data: Vec<T>,
    pub has_more: bool,
}

