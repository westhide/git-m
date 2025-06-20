use nill::{Nil, nil};

use crate::{
    cli::{Cli, Event},
    error::Result,
    event::{event_loop::EventLoop, executor::Executor},
    log::{debug, instrument},
    runtime::{
        Runtime,
        context::{Context, opts::Opts},
    },
};

#[derive(Debug)]
pub struct ExecutorImpl {}

impl ExecutorImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executor for ExecutorImpl {
    type Event = Event;

    #[instrument]
    fn execute(&self, event: Self::Event) {
        debug!("Execute: event: {event:?}");
        match event {
            Event::Init(init) => { /* TODO: Implement init event handling */ },
            Event::List(list) => { /* TODO: Implement list event handling */ },
        }
    }
}

pub fn run(cli: Cli) -> Result<Nil> {
    let opts = Opts { config: cli.config.clone() };
    let cx = Context::new(opts);
    let executor = ExecutorImpl::new();
    let evloop = EventLoop::new(executor);
    let mut rt = Runtime::new(cx, evloop);
    rt.startup();
    rt.submit(cli.event)?;

    Ok(nil)
}
