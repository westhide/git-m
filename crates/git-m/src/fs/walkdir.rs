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

use crate::{
    error::Result,
    log::{Level, instrument},
};

#[derive(Debug, Default)]
pub struct Options {
    pub batch: u32,
}

#[derive(Debug)]
struct Walk<T, F> {
    filt: F,
    rdirs: Vec<PathBuf>,
    items: Vec<T>,
}

impl<T, F> Walk<T, F> {
    const BATCH_SIZE: usize = 256;

    pub fn new<P>(path: P, filt: F) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { filt, rdirs: vec![path.into()], items: Default::default() }
    }
}

impl<T, F> FnOnce<Nil> for Walk<T, F>
where
    F: Fn(&Path) -> Result<Option<T>>,
{
    type Output = Result<Self>;

    #[instrument(level=Level::TRACE, skip_all, err)]
    extern "rust-call" fn call_once(mut self, _: Nil) -> Self::Output {
        while let Some(rdir) = self.rdirs.pop() {
            let mut rd = read_dir(rdir)?;
            while let Some(entry) = rd.next() {
                let path = entry?.path();
                if !path.is_dir() {
                    continue;
                }
                match (self.filt)(&path)? {
                    Some(item) => self.items.push(item),
                    None => self.rdirs.push(path),
                }
            }
            if self.items.len() > Self::BATCH_SIZE {
                break;
            }
        }
        Ok(self)
    }
}

#[derive(Debug)]
enum State<T, F> {
    Idle(ManuallyDrop<Walk<T, F>>),
    Pend(JoinHandle<Result<Walk<T, F>>>),
}

#[derive(Debug)]
pub struct WalkDir<T, F> {
    state: State<T, F>,
}

impl<T, F> WalkDir<T, F>
where
    // TODO: #![feature(non_lifetime_binders)]
    F: Fn(&Path) -> Result<Option<T>>,
{
    pub fn new<P>(path: P, filt: F) -> Self
    where
        P: Into<PathBuf>,
    {
        let walk = Walk::new(path, filt);
        let state = State::Idle(ManuallyDrop::new(walk));
        Self { state }
    }
}

impl<T, F> Drop for WalkDir<T, F> {
    fn drop(&mut self) {
        if let State::Idle(walk) = &mut self.state {
            unsafe { ManuallyDrop::drop(walk) }
        }
    }
}

impl<T, F> Unpin for WalkDir<T, F> {}

impl<T, F> Stream for WalkDir<T, F>
where
    T: Debug,
    T: Send + 'static,
    F: Send + 'static,
    F: Fn(&Path) -> Result<Option<T>>,
{
    type Item = Result<T>;

    #[instrument(level=Level::TRACE, skip_all, ret)]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let state = match &mut self.state {
                State::Idle(walk) => {
                    if let Some(item) = walk.items.pop() {
                        return Poll::Ready(Some(Ok(item)))
                    }
                    if walk.rdirs.nil() {
                        return Poll::Ready(None)
                    }
                    let task = unsafe { ManuallyDrop::take(walk) };
                    State::Pend(spawn_blocking(task))
                },
                State::Pend(rx) => {
                    let walk = ready!(Pin::new(rx).poll(cx))??;
                    State::Idle(ManuallyDrop::new(walk))
                },
            };
            self.state = state;
        }
    }
}
