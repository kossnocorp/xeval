use crate::prelude::*;

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
        after: Option<&str>,
        include_archived: bool,
        limit: Option<u32>,
    ) -> Result<OpenAiResponseList<OpenAiProject>, OpenAiError> {
        let client = Client::new();
        let req = client
            .get("https://api.openai.com/v1/organization/projects")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.auth.token()),
            )
            .query(&OpenAiProjectsQuery {
                after,
                include_archived,
                limit,
            });

        let resp = req
            .send()
            .await
            .map_err(OpenAiError::ProjectsRequest)?
            .error_for_status()
            .map_err(OpenAiError::ProjectsRequest)?;

        let list = resp
            .json::<OpenAiResponseList<OpenAiProject>>()
            .await
            .map_err(OpenAiError::ProjectsDeserialize)?;

        Ok(list)
    }

    pub async fn list_all_projects(
        &self,
        include_archived: bool,
    ) -> Result<Vec<OpenAiProject>, OpenAiError> {
        let mut all: Vec<OpenAiProject> = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let page = self
                .list_projects_page(after.as_deref(), include_archived, Some(100))
                .await?;

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
