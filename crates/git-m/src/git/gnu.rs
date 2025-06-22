use std::path::PathBuf;

use crate::{error::Result, git::Git};

#[derive(Debug)]
pub struct Gnu;

impl Gnu {
    pub fn new() -> Self {
        Self {}
    }
}

impl Git for Gnu {
    fn is_git_repo<P>(path: P) -> Result<bool>
    where
        P: Into<PathBuf>,
    {
        let mut dir = path.into();
        dir.push(".git");
        Ok(dir.try_exists()?)
    }
}
