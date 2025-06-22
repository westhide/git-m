use std::fmt::Debug;

use nill::{Nil, nil};
use tokio::{
    spawn,
    sync::mpsc::{Sender, channel},
    task::JoinHandle,
};

use crate::{
    error::{Result, err},
    event::execute::Execute,
    log::{error, info},
};

#[derive(Debug)]
pub struct EventLoop<E, R = Result<Nil>> {
    pub(super) tx: Option<Sender<E>>,
    pub handle: Option<JoinHandle<R>>,
}

impl<E, R> EventLoop<E, R> {
    pub fn new() -> Self {
        Self { tx: None, handle: None }
    }

    pub fn sender(&self) -> Result<&Sender<E>> {
        match &self.tx {
            Some(tx) => Ok(tx),
            None => err!("eventloop sender not exist"),
        }
    }

    pub async fn push(&mut self, event: E) -> Result<Option<E>> {
        if let Err(err) = self.sender()?.send(event).await {
            error!("eventloop push fail, {err:?}");
            Ok(Some(err.0))
        } else {
            Ok(None)
        }
    }

    pub fn release(&mut self) -> Option<Sender<E>> {
        self.tx.take()
    }
}

impl<E> EventLoop<E>
where
    E: Send + 'static,
{
    pub fn startup<T>(&mut self, mut executor: T)
    where
        T: Send + 'static,
        T: Execute<Event = E, Return = Result<Nil>>,
    {
        // TODO: Infer cpu cores
        let (tx, mut rx) = channel::<E>(10);

        let handle = spawn(async move {
            info!("eventloop running");
            while let Some(event) = rx.recv().await {
                executor.execute(event).await?;
            }
            info!("eventloop closing");
            Ok(nil)
        });

        self.tx = Some(tx);
        self.handle = Some(handle);
    }
}
