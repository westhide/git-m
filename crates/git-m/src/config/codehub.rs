use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::gdir::Gdir;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeHub {
    pub base: PathBuf,
    pub dirs: Vec<Gdir>,
}

impl CodeHub {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { base: path.into(), dirs: Default::default() }
    }

    pub fn push(&mut self, gdir: Gdir) {
        self.dirs.push(gdir);
    }
}
