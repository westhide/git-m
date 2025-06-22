pub mod context;

use std::sync::Arc;

use crate::{event::event_loop::EventLoop, runtime::context::Context};

#[derive(Debug)]
pub struct Runtime<E> {
    pub ctx: Arc<Context>,
    pub evloop: EventLoop<E>,
}

impl<E> Runtime<E>
where
    E: Send + 'static,
{
    pub fn new(ctx: Context, evloop: EventLoop<E>) -> Self {
        Self { ctx: Arc::new(ctx), evloop }
    }
}
