use std::{fmt::Debug, mem::ManuallyDrop};

use tokio::{
    spawn,
    sync::mpsc::{Sender, channel},
    task::JoinHandle,
};

use crate::{
    error::Result,
    event::execute::Execute,
    log::{error, info},
};

#[derive(Debug)]
pub struct EventLoop<E, R> {
    tx: Sender<E>,
    hd: JoinHandle<R>,
}

impl<T, EE, ER> EventLoop<EE::Event, Result<EE, ER>>
where
    EE: Send + 'static,
    ER: Send + 'static,
    EE: Execute<Return = Result<T, ER>>,
    EE::Event: Send,
{
    pub fn startup(mut executor: EE) -> Self {
        // TODO: Infer cpu cores
        let (tx, mut rx) = channel(10);

        let hd = spawn(async move {
            info!("eventloop running");
            while let Some(event) = rx.recv().await {
                executor.execute(event).await?;
            }
            info!("eventloop closing");
            Ok(executor)
        });

        Self { tx, hd }
    }
}

impl<E, R> EventLoop<E, R> {
    pub fn sender(&self) -> &Sender<E> {
        &self.tx
    }

    pub async fn push(&mut self, event: E) -> Option<E> {
        if let Err(err) = self.tx.send(event).await {
            error!("eventloop push fail, {err:?}");
            Some(err.0)
        } else {
            None
        }
    }

    pub fn into_handle(self) -> JoinHandle<R> {
        self.hd
    }
}
