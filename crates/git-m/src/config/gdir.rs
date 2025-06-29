use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::git::IGit;

#[derive(Debug, Serialize, Deserialize)]
pub struct Gdir {
    pub path: PathBuf,
}

impl Gdir {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { path: path.into() }
    }
}

impl<T> From<T> for Gdir
where
    T: IGit,
{
    fn from(git: T) -> Self {
        Self::new(git.workdir())
    }
}
