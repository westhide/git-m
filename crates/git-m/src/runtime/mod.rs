pub mod context;

use nill::Nil;

use crate::{
    error::Result,
    event::{event_loop::EventLoop, executor::Executor},
    log::info,
    runtime::context::Context,
};

#[derive(Debug)]
pub struct Runtime<E, EE>
where
    EE: Executor<Event = E, Return = Result<Nil>>,
{
    pub ctx: Context,
    pub evloop: EventLoop<E, EE>,
}

impl<E, EE> Runtime<E, EE>
where
    E: Send + 'static,
    EE: Executor<Event = E, Return = Result<Nil>> + Send + 'static,
{
    pub fn new(ctx: Context, evloop: EventLoop<E, EE>) -> Self {
        Self { ctx, evloop }
    }

    pub fn run(&mut self) {
        info!("runtime running");
        self.evloop.start();
    }

    pub fn release(&mut self) {
        let _sender = self.evloop.release();
    }

    pub async fn submit(&mut self, event: E) -> Result<Option<E>> {
        self.evloop.push(event).await
    }
}
