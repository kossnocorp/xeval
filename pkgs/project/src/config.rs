use crate::prelude::*;
use std::fs;

pub const CONFIG_FILENAME: &str = "xeval.toml";

pub const DEFAULT_EVALS_GLOB: &str = "./evals/**/*.yaml";

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to find config file at {0}")]
    NotFound(PathBuf),

    #[error("Failed to read config file")]
    ReadFailed(#[from] config::ConfigError),

    #[error("Config file already exists, pass --force to overwrite")]
    AlreadyExists(PathBuf),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Glob pattern to find eval YAML files
    #[serde(default = "Config::default_evals_glob")]
    pub evals: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            evals: Self::default_evals_glob(),
        }
    }
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

        Ok(Default::default())
    }

    pub fn find(path: &Option<PathBuf>) -> Result<Config, ConfigError> {
        match path {
            // Strict mode: if caller passed a path, don't walk up — use it directly
            Some(p) => {
                if p.is_dir() {
                    let file = Self::resolve_path(p);
                    return Self::read(file);
                } else {
                    return Self::read(p.clone());
                }
            }
            None => {
                // Legacy behavior: search upwards from current directory
                let path: PathBuf = ".".into();
                let mut current = Some(path.as_path());
                while let Some(dir) = current {
                    let file = dir.join(CONFIG_FILENAME);
                    if file.is_file() {
                        return Self::read(file);
                    }
                    current = dir.parent();
                }
                Err(ConfigError::NotFound(path))
            }
        }
    }

    fn read(path: PathBuf) -> Result<Config, ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::from(path))
            .add_source(config::Environment::with_prefix("XEVAL"))
            .build()?;

        Ok(settings.try_deserialize::<Config>()?)
    }

    pub fn write(&self, project: &Project) -> Result<(), ConfigError> {
        let toml = toml::to_string_pretty(self).map_err(|e| {
            config::ConfigError::Message(format!("Failed to serialize config: {e}"))
        })?;
        fs::write(&project.get_config_path(), toml)
            .map_err(|e| config::ConfigError::Message(format!("Failed to write config: {e}")))?;
        Ok(())
    }

    pub fn write_new(path: &PathBuf, force: bool, config: &Config) -> Result<(), ConfigError> {
        let dest = Self::resolve_path(path);
        if dest.exists() && !force {
            return Err(ConfigError::AlreadyExists(dest));
        }
        let parent = dest
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        if !parent.exists() {
            fs::create_dir_all(&parent).map_err(|e| {
                config::ConfigError::Message(format!("Failed to create directories: {e}"))
            })?;
        }
        let toml = toml::to_string_pretty(config).map_err(|e| {
            config::ConfigError::Message(format!("Failed to serialize config: {e}"))
        })?;
        fs::write(&dest, toml)
            .map_err(|e| config::ConfigError::Message(format!("Failed to write config: {e}")))?;
        Ok(())
    }

    pub fn default_evals_glob() -> String {
        DEFAULT_EVALS_GLOB.to_string()
    }
}

impl Project {
    fn get_config_path(&self) -> PathBuf {
        match self.config_path {
            Some(ref path) => path.clone(),
            None => self.path.join(CONFIG_FILENAME),
        }
    }
}
