use std::fmt::Debug;

use nill::{Nil, nil};
use tokio::{
    sync::mpsc::{Sender, channel},
    task::JoinHandle,
};

use crate::{
    error::{Result, err},
    event::executor::Executor,
    log::{error, info},
};

#[derive(Debug)]
pub struct EventLoop<E, EE>
where
    EE: Executor<Event = E, Return = Result<Nil>>,
{
    executor: EE,
    tx: Option<Sender<E>>,

    pub handle: Option<JoinHandle<EE::Return>>,
}

impl<E, EE> EventLoop<E, EE>
where
    E: Send + 'static,
    EE: Executor<Event = E, Return = Result<Nil>> + Send + 'static,
{
    pub fn new(executor: EE) -> Self {
        Self { tx: None, executor, handle: None }
    }

    pub fn start(&mut self) {
        info!("starting eventloop");

        // TODO: Infer cpu cores
        let (tx, mut rx) = channel::<E>(10);

        let executor = self.executor.clone();
        let handle = tokio::spawn(async move {
            info!("eventloop running");
            while let Some(event) = rx.recv().await {
                executor.execute(event)?;
            }
            info!("eventloop closing");
            Ok(nil)
        });

        self.tx = Some(tx);
        self.handle = Some(handle);
    }

    pub fn release(&mut self) -> Option<Sender<E>> {
        self.tx.take()
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
}
