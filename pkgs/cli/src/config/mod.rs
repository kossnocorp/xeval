use crate::prelude::*;

pub const CONFIG_FILENAME: &str = "xeval.toml";

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to find config file at {0}")]
    NotFound(PathBuf),

    #[error("Failed to read config file")]
    ReadFailed(#[from] config::ConfigError),

    #[error("Config file already exists, pass --force to overwrite")]
    AlreadyExists(PathBuf),
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    openai: OpenAiConfig,
}

impl Config {
    pub fn resolve_path(path: &PathBuf) -> PathBuf {
        if path.ends_with(CONFIG_FILENAME) {
            return path.clone();
        }

        path.join(CONFIG_FILENAME)
    }

    pub fn init(path: &PathBuf, force: bool) -> Result<Self, ConfigError> {
        let path = Self::resolve_path(path);

        if path.exists() && !force {
            return Err(ConfigError::AlreadyExists(path.clone()));
        }

        // TODO: Authenticate with OpenAI

        Ok(Default::default())
    }

    pub fn find(path: &Option<PathBuf>) -> Result<Config, ConfigError> {
        let path: PathBuf = path.clone().unwrap_or_else(|| ".".into());

        let mut current = if path.is_dir() {
            Some(path.as_path())
        } else {
            path.parent()
        };

        while let Some(dir) = current {
            let file = dir.join(CONFIG_FILENAME);
            if file.is_file() {
                return Self::read(file);
            }
            current = dir.parent();
        }

        Err(ConfigError::NotFound(path.clone()))
    }

    fn read(path: PathBuf) -> Result<Config, ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::from(path))
            .add_source(config::Environment::with_prefix("XEVAL"))
            .build()?;

        Ok(settings.try_deserialize::<Config>()?)
    }
}
