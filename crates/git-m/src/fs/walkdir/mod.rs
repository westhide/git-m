pub mod iter;
pub mod stream;

use std::{
    fmt::Debug,
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::{
    error::Result,
    fs::walkdir::{iter::WalkDirIter, stream::WalkDirStream},
};

#[derive(Debug, Default)]
pub struct Options {
    pub depth: u32,
}

#[derive(Debug)]
pub struct WalkDir<F> {
    filter: F,
    rdirs: Vec<PathBuf>,
}

impl<F> WalkDir<F>
where
    F: Fn(&Path) -> Result<bool>,
{
    pub fn new<P>(path: P, filter: F) -> Self
    where
        P: Into<PathBuf>,
    {
        WalkDir { filter, rdirs: vec![path.into()] }
    }

    pub fn find_repo_dirs(self) -> Result<Vec<PathBuf>> {
        let Self { mut rdirs, .. } = self;
        let mut repos = vec![];

        while let Some(dir) = rdirs.pop() {
            let mut rd = read_dir(dir)?;
            while let Some(entry) = rd.next() {
                let path = entry?.path();
                if !path.is_dir() {
                    continue;
                }
                if (self.filter)(&path)? {
                    repos.push(path);
                } else {
                    rdirs.push(path);
                }
            }
        }

        Ok(repos)
    }

    pub fn into_iter(self) -> WalkDirIter<F> {
        WalkDirIter::new(self.rdirs[0].clone(), self.filter)
    }

    pub fn into_stream(self) -> WalkDirStream<F> {
        WalkDirStream::new(self.rdirs[0].clone(), self.filter)
    }
}
