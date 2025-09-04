use std::fs;

use crate::prelude::*;

pub const GLOBAL_AUTH_FILENAME: &str = "auth.toml";

#[derive(Error, Debug)]
pub enum GlobalAuthError {
    #[error("Failed to parse {0}")]
    Parse(PathBuf),

    #[error("Failed to update {0} key")]
    Update(String),

    #[error("Failed to write {0}")]
    Write(PathBuf),
}

pub struct GlobalAuth {
    doc: DocumentMut,
}

pub type GlobalAuthUpdate = fn(&mut Table) -> Result<()>;

impl GlobalAuth {
    fn resolve_path(dir: &GlobalDir) -> PathBuf {
        dir.resolve_path(GLOBAL_AUTH_FILENAME)
    }

    pub fn resolve(dir: &GlobalDir) -> Result<Self, GlobalAuthError> {
        let path = Self::resolve_path(dir);
        let doc = fs::read_to_string(&path).map_or_else(
            |_| Ok(DocumentMut::new()),
            |str| {
                str.parse::<DocumentMut>()
                    .map_err(|_| GlobalAuthError::Parse(path))
            },
        )?;
        Ok(GlobalAuth { doc })
    }

    pub fn get(&self, name: &str) -> Option<&Table> {
        self.doc.get(name).and_then(|item| item.as_table())
    }

    pub fn update<UpdateFn>(&mut self, name: &str, update: UpdateFn) -> Result<()>
    where
        UpdateFn: FnOnce(&mut Table) -> Result<()>,
    {
        let mut table = self
            .doc
            .entry(name)
            .or_insert(Item::Table(Table::new()))
            .as_table_mut()
            .with_context(|| format!("Failed to access {name} as table"))?;
        update(&mut table)?;
        Ok(())
    }

    pub fn persist(&self, dir: &GlobalDir) -> Result<(), GlobalAuthError> {
        let path = Self::resolve_path(dir);
        fs::write(&path, self.doc.to_string()).map_err(|_| GlobalAuthError::Write(path))?;
        Ok(())
    }
}
