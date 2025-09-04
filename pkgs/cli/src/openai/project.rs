use crate::prelude::*;

pub struct OpenAiLocalProject {}

impl OpenAiLocalProject {
    pub async fn select(auth: &Auth) -> Result<()> {
        let spinner = UiTheme::start_spinner("Loading OpenAI projects");

        let projects = auth.openai.list_all_projects(false).await?;

        println!("PROJECTS {:?}", projects);

        spinner.finish_and_clear();

        Ok(())
    }
}
