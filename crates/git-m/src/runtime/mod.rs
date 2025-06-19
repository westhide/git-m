pub mod context;

use std::sync::mpsc::SendError;

use nill::Nil;

use crate::{
    event::{event_loop::EventLoop, executor::Executor},
    runtime::context::Context,
};

#[derive(Debug)]
pub struct Runtime<E, EE> {
    pub ctx: Context,
    pub evloop: EventLoop<E, EE>,
}

impl<E, EE> Runtime<E, EE>
where
    EE: Executor<Event = E>,
{
    pub fn new(ctx: Context, evloop: EventLoop<E, EE>) -> Self {
        Self { ctx, evloop }
    }

    pub fn start(&mut self) {
        self.evloop.start()
    }

    pub fn run(&mut self, event: E) -> Result<Nil, SendError<E>> {
        self.evloop.push(event)
    }
}
