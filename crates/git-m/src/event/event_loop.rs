use std::sync::mpsc::{Receiver, SendError, Sender, channel};

use nill::Nil;

use crate::event::executor::Executor;

#[derive(Debug)]
pub struct EventLoop<E, EE> {
    rx: Receiver<E>,
    tx: Sender<E>,
    executor: EE,
}

impl<E, EE> EventLoop<E, EE>
where
    EE: Executor<Event = E>,
{
    pub fn new(executor: EE) -> Self {
        let (tx, rx) = channel::<E>();

        Self { tx, rx, executor }
    }

    pub fn start(&mut self) {
        while let Ok(event) = self.rx.recv() {
            self.executor.execute(event);
        }
    }

    pub fn sender(&self) -> Sender<E> {
        self.tx.clone()
    }

    pub fn push(&mut self, event: E) -> Result<Nil, SendError<E>> {
        self.tx.send(event)
    }
}
