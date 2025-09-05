use crate::prelude::*;

#[derive(Error, Debug)]
pub enum OpenAiProjectsError {
    #[error("Failed to perform projects request: {0}")]
    Request(reqwest::Error),

    #[error("Failed to parse projects response: {0}")]
    Deserialize(reqwest::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiProject {
    pub id: String,
    pub object: String,
    pub name: String,
    pub status: String,
    pub created_at: i64,
    pub archived_at: Option<i64>,
}

#[derive(Serialize)]
struct OpenAiProjectsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<&'a str>,
    include_archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl OpenAi {
    pub async fn list_projects_page(
        &self,
        params: &OpenAiListProjectsPageRequest,
    ) -> Result<OpenAiResponseList<OpenAiProject>, OpenAiProjectsError> {
        let client = Client::new();
        let req = client
            .get("https://api.openai.com/v1/organization/projects")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.auth.token()),
            )
            .query(&OpenAiProjectsQuery {
                after: params.after.as_deref(),
                include_archived: params.include_archived,
                limit: params.limit,
            });

        let resp = req
            .send()
            .await
            .map_err(OpenAiProjectsError::Request)?
            .error_for_status()
            .map_err(OpenAiProjectsError::Request)?;

        let list = resp
            .json::<OpenAiResponseList<OpenAiProject>>()
            .await
            .map_err(OpenAiProjectsError::Deserialize)?;

        Ok(list)
    }

    pub async fn list_all_projects(
        &self,
        params: OpenAiListAllProjectsRequest,
    ) -> Result<Vec<OpenAiProject>, OpenAiProjectsError> {
        let mut all: Vec<OpenAiProject> = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let page_params = OpenAiListProjectsPageRequest {
                after: after.clone(),
                include_archived: params.include_archived,
                limit: Some(100),
            };

            let page = self.list_projects_page(&page_params).await?;

            let has_more = page.has_more;
            if page.data.is_empty() {
                break;
            }

            after = page.data.last().map(|p| p.id.clone());
            all.extend(page.data);

            if !has_more {
                break;
            }
        }

        Ok(all)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAiListProjectsPageRequest {
    pub after: Option<String>,
    pub include_archived: bool,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenAiListAllProjectsRequest {
    pub include_archived: bool,
}
