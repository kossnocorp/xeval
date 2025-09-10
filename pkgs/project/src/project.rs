use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Project {
    pub path: PathBuf,
    pub config: Config,
    pub config_path: Option<PathBuf>,
}

impl Project {
    pub fn from_config(config: Config, config_path: Option<PathBuf>) -> Self {
        Self {
            path,
            config,
            config_path,
        }
    }
}
