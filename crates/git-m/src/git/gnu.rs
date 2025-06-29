use std::{fmt::Debug, path::Path};

use libgit::{ErrorCode, Repository as Gdir};

use crate::{error::Result, git::IGit};

pub struct Git {
    gdir: Gdir,
}

impl Git {
    pub fn new(gdir: Gdir) -> Self {
        Self { gdir }
    }
}

impl Debug for Git {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Git").field("gdir", &self.gdir.workdir()).finish()
    }
}

impl IGit for Git {
    fn open(path: &Path) -> Result<Option<Self>> {
        match Gdir::open(path) {
            Ok(gdir) => Ok(Some(Self::new(gdir))),
            Err(err) if matches!(err.code(), ErrorCode::NotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    fn workdir(&self) -> &Path {
        match self.gdir.workdir() {
            Some(dir) => dir,
            None => todo!("bare git dir"),
        }
    }
}
