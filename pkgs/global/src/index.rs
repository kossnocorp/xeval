use crate::prelude::*;

#[derive(Error, Debug)]
pub enum GlobalError {
    #[error(transparent)]
    Dir(#[from] GlobalDirError),

    #[error(transparent)]
    Auth(#[from] GlobalAuthError),
}

pub struct Global {
    dir: GlobalDir,
    pub auth: GlobalAuth,
}

impl Global {
    pub fn resolve() -> Result<Self, GlobalError> {
        let dir = GlobalDir::resolve()?;
        let auth = GlobalAuth::resolve(&dir)?;
        Ok(Global { dir, auth })
    }

    pub fn persist_auth(&self) -> Result<(), GlobalError> {
        self.auth.persist(&self.dir)?;
        Ok(())
    }
}
