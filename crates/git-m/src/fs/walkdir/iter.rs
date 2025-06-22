use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::error::Result;

#[derive(Debug)]
pub struct WalkDirIter<F> {
    filter: F,
    rdirs: Vec<PathBuf>,
    repos: Vec<PathBuf>,
}

impl<F> WalkDirIter<F> {
    pub fn new<P>(path: P, filter: F) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { filter, rdirs: vec![path.into()], repos: vec![] }
    }
}

impl<F> WalkDirIter<F>
where
    F: Fn(&Path) -> Result<bool>,
{
    fn read_next(&mut self) -> Result<Option<PathBuf>> {
        loop {
            if let Some(dir) = self.rdirs.pop() {
                let mut rd = read_dir(dir)?;
                while let Some(entry) = rd.next() {
                    let path = entry?.path();
                    if !path.is_dir() {
                        continue;
                    }
                    if (self.filter)(&path)? {
                        self.repos.push(path);
                    } else {
                        self.rdirs.push(path);
                    }
                }
            } else {
                return Ok(None)
            }
            if let Some(repo) = self.repos.pop() {
                return Ok(Some(repo));
            }
        }
    }
}

impl<F> Iterator for WalkDirIter<F>
where
    F: Fn(&Path) -> Result<bool>,
{
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(repo) = self.repos.pop() {
            return Some(Ok(repo));
        }
        self.read_next().transpose()
    }
}
