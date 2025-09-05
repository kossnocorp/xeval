use crate::prelude::*;
use litty::literal;
use ordered_float::OrderedFloat;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

//#region OpenAiEval

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiEval {
    pub object: OpenAiEvalObject,
    pub id: String,
    pub name: String,
    pub created_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    pub data_source_config: OpenAiDataSourceConfig,
    pub testing_criteria: Vec<OpenAiGrader>,
}

#[literal("eval")]
pub struct OpenAiEvalObject;

//#region OpenAiDataSourceConfig

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum OpenAiDataSourceConfig {
    Custom(OpenAiCustomDataSourceConfig),
    Logs(OpenAiLogsDataSourceConfig),
    StoredCompletions(OpenAiStoredCompletionsDataSourceConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiCustomDataSourceConfig {
    #[serde(rename = "type")]
    pub r#type: OpenAiCustomDataSourceConfigType,
    pub schema: Value,
}

#[literal("custom")]
pub struct OpenAiCustomDataSourceConfigType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiLogsDataSourceConfig {
    #[serde(rename = "type")]
    pub r#type: OpenAiLogsDataSourceConfigType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
}

#[literal("logs")]
pub struct OpenAiLogsDataSourceConfigType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiStoredCompletionsDataSourceConfig {
    #[serde(rename = "type")]
    pub r#type: OpenAiStoredCompletionsDataSourceConfigType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
}

#[literal("stored_completions")]
pub struct OpenAiStoredCompletionsDataSourceConfigType;

//#endregion

//#region OpenAiTestingCriteria

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum OpenAiGrader {
    StringCheck(OpenAiGraderStringCheck),
    TextSimilarity(OpenAiGraderTextSimilarity),
    LabelModel(OpenAiGraderLabelModel),
    ScoreModel(OpenAiGraderScoreModel),
    Python(OpenAiGraderPython),
}

//#region OpenAiGraderStringCheck

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiGraderStringCheck {
    #[serde(rename = "type")]
    pub r#type: OpenAiGraderStringCheckType,
    pub name: String,
    pub operation: OpenAiGraderStringCheckOperation,
    pub input: String,
    pub reference: String,
}

#[literal("string_check")]
pub struct OpenAiGraderStringCheckType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiGraderStringCheckOperation {
    Eq,
    Ne,
    Like,
    Ilike,
}

//#endregion

//#region OpenAiGraderTextSimilarity

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiGraderTextSimilarity {
    #[serde(rename = "type")]
    pub r#type: OpenAiGraderTextSimilarityType,
    pub name: String,
    pub evaluation_metric: OpenAiGraderTextSimilarityEvaluationMetric,
    pub input: String,
    pub reference: String,
    pub pass_threshold: OrderedFloat<f64>,
}

#[literal("text_similarity")]
pub struct OpenAiGraderTextSimilarityType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiGraderTextSimilarityEvaluationMetric {
    Cosine,
    FuzzyMatch,
    Bleu,
    Gleu,
    Meteor,
    #[serde(rename = "rouge_1")]
    Rouge1,
    #[serde(rename = "rouge_2")]
    Rouge2,
    #[serde(rename = "rouge_3")]
    Rouge3,
    #[serde(rename = "rouge_4")]
    Rouge4,
    #[serde(rename = "rouge_5")]
    Rouge5,
    RougeL,
}

//#endregion

//#region OpenAiGraderScoreModel

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiGraderScoreModel {
    #[serde(rename = "type")]
    pub r#type: OpenAiGraderScoreModelType,
    pub name: String,
    pub model: String,
    pub pass_threshold: OrderedFloat<f64>,
    pub range: [OrderedFloat<f64>; 2],
    pub sampling_params: Map<String, Value>,
    pub input: Vec<OpenAiModelInput>,
}

#[literal("score_model")]
pub struct OpenAiGraderScoreModelType;

//#endregion

//#region OpenAiGraderLabelModel

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiGraderLabelModel {
    #[serde(rename = "type")]
    pub r#type: OpenAiGraderLabelModelType,
    pub name: String,
    pub model: String,
    pub labels: Vec<String>,
    pub passing_labels: Vec<String>,
    pub input: Vec<OpenAiModelInput>,
}

#[literal("label_model")]
pub struct OpenAiGraderLabelModelType;

//#endregion

//#region OpenAiGraderPython

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiGraderPython {
    #[serde(rename = "type")]
    pub r#type: OpenAiGraderPythonType,
    pub name: String,
    pub source: String,
    pub image_tag: String,
    pub pass_threshold: OrderedFloat<f64>,
}

#[literal("python")]
pub struct OpenAiGraderPythonType;

//#endregion

//#region OpenAiModelInput

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiModelInput {
    #[serde(rename = "type")]
    pub r#type: OpenAiMessageType,
    pub role: OpenAiMessageRole,
    pub content: OpenAiModelInputContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum OpenAiModelInputContent {
    String(String),
    InputText(OpenAiModelInputItemText),
    InputImage(OpenAiModelInputItemImage),
    InputAudio(OpenAiModelInputItemAudio),
    OutputText(OpenAiModelOutputItemText),
    Array(Vec<OpenAiModelInputItem>),
}

//#region OpenAiModelInputItem

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum OpenAiModelInputItem {
    Text(OpenAiModelInputItemText),
    Image(OpenAiModelInputItemImage),
    Audio(OpenAiModelInputItemAudio),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiModelInputItemText {
    #[serde(rename = "type")]
    pub r#type: OpenAiModelInputItemTextType,
    pub text: String,
}

#[literal("input_text")]
pub struct OpenAiModelInputItemTextType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiModelInputItemImage {
    #[serde(rename = "type")]
    pub r#type: OpenAiModelInputItemImageType,
    pub image_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<OpenAiModelInputItemImageDetail>,
}

#[literal("input_image")]
pub struct OpenAiModelInputItemImageType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiModelInputItemImageDetail {
    High,
    Low,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiModelInputItemAudio {
    #[serde(rename = "type")]
    pub r#type: OpenAiModelInputItemAudioType,
    pub input_audio: OpenAiModelInputItemAudioData,
}

#[literal("input_audio")]
pub struct OpenAiModelInputItemAudioType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiModelInputItemAudioData {
    pub data: String,   // base64-encoded audio data
    pub format: String, // e.g. "mp3", "wav"
}

//#endregion

//#region OpenAiModelOutputItem

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct OpenAiModelOutputItemText {
    #[serde(rename = "type")]
    pub r#type: OpenAiModelOutputItemTextType,
    pub text: String,
}

#[literal("output_text")]
pub struct OpenAiModelOutputItemTextType;

//#endregion

//#endregion

//#endregion

//#endregion

//#region OpenAiMessage

#[literal("message")]
pub struct OpenAiMessageType;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiMessageRole {
    User,
    Assistant,
    System,
    Developer,
}

//#endregion

#[derive(Error, Debug)]
pub enum OpenAiEvalsError {
    #[error("Failed to perform evals request: {0}")]
    Request(reqwest::Error),

    #[error("Failed to obtain body text: {0}")]
    Body(reqwest::Error),

    #[error("Failed to parse evals response: {0}")]
    Deserialize(serde_json::Error),
}

#[derive(Serialize)]
struct OpenAiEvalsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<&'a str>,
}

impl OpenAi {
    pub async fn list_evals_page(
        &self,
        params: &OpenAiListEvalsPageRequest,
    ) -> Result<OpenAiResponseList<OpenAiEval>, OpenAiEvalsError> {
        let client = Client::new();
        let mut req = client.get("https://api.openai.com/v1/evals").header(
            header::AUTHORIZATION,
            format!("Bearer {}", self.auth.token()),
        );

        if let Some(project) = params.project.as_deref().filter(|p| !p.is_empty()) {
            req = req.header("OpenAI-Project", project);
        }

        req = req.query(&OpenAiEvalsQuery {
            after: params.after.as_deref(),
            limit: params.limit,
            order: params.order.as_deref(),
            order_by: params.order_by.as_deref(),
        });

        let resp = req
            .send()
            .await
            .map_err(OpenAiEvalsError::Request)?
            .error_for_status()
            .map_err(OpenAiEvalsError::Request)?;

        let text = resp
            .text()
            .await
            .map_err(|err| OpenAiEvalsError::Body(err))?;

        let list = serde_json::from_str::<OpenAiResponseList<OpenAiEval>>(&text)
            .map_err(|err| OpenAiEvalsError::Deserialize(err))?;

        Ok(list)
    }

    pub async fn list_all_evals(
        &self,
        params: &OpenAiListAllEvalsRequest,
    ) -> Result<Vec<OpenAiEval>, OpenAiEvalsError> {
        let mut all: Vec<OpenAiEval> = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let page_params = OpenAiListEvalsPageRequest {
                project: params.project.clone(),
                after: after.clone(),
                limit: Some(100),
                order: params.order.clone(),
                order_by: params.order_by.clone(),
            };

            let page = self.list_evals_page(&page_params).await?;

            let has_more = page.has_more;
            if page.data.is_empty() {
                break;
            }

            after = page.data.last().map(|e| e.id.clone());
            all.extend(page.data);

            if !has_more {
                break;
            }
        }

        Ok(all)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAiListEvalsPageRequest {
    pub project: Option<String>,
    pub after: Option<String>,
    pub limit: Option<u32>,
    pub order: Option<String>,
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAiListAllEvalsRequest {
    pub project: Option<String>,
    pub order: Option<String>,
    pub order_by: Option<String>,
}
