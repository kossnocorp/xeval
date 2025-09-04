use crate::prelude::*;

pub const GLOBAL_DIRNAME: &str = "xeval";

#[derive(Error, Debug)]
pub enum GlobalDirError {
    #[error("Failed to resolve home directory")]
    ResolveHome,

    #[error("Failed to create global directory: {0}")]
    Create(#[from] std::io::Error),
}

pub struct GlobalDir {
    path: PathBuf,
}

impl GlobalDir {
    pub fn resolve() -> Result<Self, GlobalDirError> {
        let home_path = dirs::home_dir().ok_or(GlobalDirError::ResolveHome)?;
        let path = home_path.join(".local").join("state").join(GLOBAL_DIRNAME);
        let _ = std::fs::create_dir_all(&path).map_err(|err| GlobalDirError::Create(err))?;
        Ok(Self { path })
    }

    pub fn resolve_path(&self, path: &str) -> PathBuf {
        self.path.join(path)
    }
}
