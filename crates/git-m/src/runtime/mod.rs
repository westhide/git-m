pub mod context;

use std::sync::Arc;

use crate::runtime::context::Context;

#[derive(Debug)]
pub struct Runtime {
    pub ctx: Arc<Context>,
}

impl Runtime {
    pub fn new(ctx: Context) -> Self {
        Self { ctx: Arc::new(ctx) }
    }
}
