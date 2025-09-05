use crate::prelude::*;
use std::path::PathBuf as StdPathBuf;

pub const OPENAI_DIRNAME: &str = "openai";
pub const OPENAI_EVALS_FILENAME: &str = "evals.json";

#[derive(Debug)]
pub struct OpenAiLocalProjectState<'a> {
    pub evals: ProjectStateFile<'a>,
}

impl<'a> OpenAiLocalProjectState<'a> {
    pub fn new(project: &'a Project) -> Result<Self> {
        let relative: StdPathBuf = StdPathBuf::from(OPENAI_DIRNAME).join(OPENAI_EVALS_FILENAME);
        let file = ProjectStateFile::ensure(project, relative)?;
        Ok(Self { evals: file })
    }

    pub fn evals_path(&self) -> &PathBuf {
        &self.evals.path
    }
}
