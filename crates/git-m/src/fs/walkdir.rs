use std::{
    fmt::Debug,
    fs::read_dir,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
    pin::Pin,
    task::{Context, Poll, ready},
};

use futures::Stream;
use nill::{Nil, Nill};
use tokio::task::{JoinHandle, spawn_blocking};

use crate::error::Result;

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

    pub fn into_stream(self) -> WalkDirStream<F> {
        let Self { filter, rdirs } = self;
        let walker = Walker { filter, rdirs, repos: vec![] };
        let state = State::Idle(ManuallyDrop::new(walker));
        WalkDirStream { state }
    }
}

#[derive(Debug)]
pub struct WalkDirStream<F> {
    state: State<F>,
}

impl<F> Drop for WalkDirStream<F> {
    fn drop(&mut self) {
        if let State::Idle(walker) = &mut self.state {
            unsafe { ManuallyDrop::drop(walker) }
        }
    }
}

impl<F> Unpin for WalkDirStream<F> {}

#[derive(Debug)]
enum State<F> {
    Idle(ManuallyDrop<Walker<F>>),
    Pend(JoinHandle<Result<Walker<F>>>),
}

#[derive(Debug)]
struct Walker<F> {
    filter: F,
    rdirs: Vec<PathBuf>,
    repos: Vec<PathBuf>,
}

impl<F> FnOnce<Nil> for Walker<F>
where
    F: Fn(&Path) -> Result<bool>,
{
    type Output = Result<Self>;

    extern "rust-call" fn call_once(mut self, _: Nil) -> Self::Output {
        use std::fs::read_dir;
        while let Some(dir) = self.rdirs.pop() {
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
        }
        Ok(self)
    }
}

impl<F> Stream for WalkDirStream<F>
where
    F: Send + 'static,
    F: Fn(&Path) -> Result<bool>,
{
    type Item = Result<PathBuf>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let state = match &mut self.state {
                State::Idle(walker) => {
                    if let Some(repo) = walker.repos.pop() {
                        return Poll::Ready(Some(Ok(repo)))
                    }
                    if walker.rdirs.nil() {
                        return Poll::Ready(None)
                    }
                    let task = unsafe { ManuallyDrop::take(walker) };
                    State::Pend(spawn_blocking(task))
                },
                State::Pend(rx) => {
                    let walker = ready!(Pin::new(rx).poll(cx))??;
                    State::Idle(ManuallyDrop::new(walker))
                },
            };
            self.state = state;
        }
    }
}
