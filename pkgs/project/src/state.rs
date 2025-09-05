use crate::prelude::*;
use std::fs;
use std::path::Path;
use toml_edit::Document;
use serde::de::DeserializeOwned;
use serde::Serialize as SerSerialize;

pub const PROJECT_STATE_DIRNAME: &str = ".xeval";

#[derive(Debug, Clone)]
pub struct ProjectStateFile<'a> {
    pub project: &'a Project,
    pub path: PathBuf,
}

impl<'a> ProjectStateFile<'a> {
    /// Ensure the parent directory exists and return a state file handle.
    pub fn ensure<PathType: AsRef<Path>>(project: &'a Project, relative: PathType) -> Result<Self> {
        let base = project.path.join(PROJECT_STATE_DIRNAME);
        let path = base.join(relative.as_ref());
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create project state dir: {}", parent.display()))?;
        }
        Ok(Self { project, path })
    }

    /// Write a TOML document to the state file path.
    pub fn write<S>(&self, doc: &Document<S>) -> Result<()>
    where
        Document<S>: std::fmt::Display,
    {
        let s = doc.to_string();
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create project state dir: {}", parent.display()))?;
        }
        fs::write(&self.path, s)
            .with_context(|| format!("Failed to write state file: {}", self.path.display()))?;
        Ok(())
    }

    /// Read a TOML file into T or return T::default() if file doesn't exist.
    pub fn read_toml_or_default<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Default,
    {
        if Path::new(&self.path).exists() {
            let text = fs::read_to_string(&self.path).with_context(|| {
                format!("Failed to read state file: {}", self.path.display())
            })?;
            let value = toml::from_str::<T>(&text).with_context(|| {
                format!("Failed to deserialize state file: {}", self.path.display())
            })?;
            Ok(value)
        } else {
            Ok(Default::default())
        }
    }

    /// Serialize `data` to TOML and write to the state file path.
    pub fn write_toml<T: SerSerialize>(&self, data: &T) -> Result<()> {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create project state dir: {}", parent.display()))?;
        }
        let s = toml::to_string_pretty(data).context("Failed to serialize TOML")?;
        fs::write(&self.path, s)
            .with_context(|| format!("Failed to write state file: {}", self.path.display()))?;
        Ok(())
    }

    /// Read YAML into T or return default.
    pub fn read_yaml_or_default<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Default,
    {
        if Path::new(&self.path).exists() {
            let text = fs::read_to_string(&self.path).with_context(|| {
                format!("Failed to read state file: {}", self.path.display())
            })?;
            let value = serde_yaml::from_str::<T>(&text).with_context(|| {
                format!("Failed to deserialize YAML: {}", self.path.display())
            })?;
            Ok(value)
        } else {
            Ok(Default::default())
        }
    }

    /// Serialize to YAML and write.
    pub fn write_yaml<T>(&self, data: &T) -> Result<()>
    where
        T: SerSerialize,
    {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create project state dir: {}", parent.display()))?;
        }
        let s = serde_yaml::to_string(data).context("Failed to serialize YAML")?;
        fs::write(&self.path, s)
            .with_context(|| format!("Failed to write state file: {}", self.path.display()))?;
        Ok(())
    }

    /// Read JSON into T or return default.
    pub fn read_json_or_default<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Default,
    {
        if Path::new(&self.path).exists() {
            let text = fs::read_to_string(&self.path).with_context(|| {
                format!("Failed to read state file: {}", self.path.display())
            })?;
            let value = serde_json::from_str::<T>(&text).with_context(|| {
                format!("Failed to deserialize JSON: {}", self.path.display())
            })?;
            Ok(value)
        } else {
            Ok(Default::default())
        }
    }

    /// Serialize to JSON and write.
    pub fn write_json<T>(&self, data: &T) -> Result<()>
    where
        T: SerSerialize,
    {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create project state dir: {}", parent.display()))?;
        }
        let s = serde_json::to_string_pretty(data).context("Failed to serialize JSON")?;
        fs::write(&self.path, s)
            .with_context(|| format!("Failed to write state file: {}", self.path.display()))?;
        Ok(())
    }
}
