use crate::{
    cli::{Cli, Event as CliEvent},
    event::{event_loop::EventLoop, executor::Executor},
    runtime::{
        Runtime,
        context::{Context, opts::Opts},
    },
};

pub struct ExecutorImpl {}

impl ExecutorImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executor for ExecutorImpl {
    type Event = CliEvent;

    fn execute(&self, event: Self::Event) {
        match event {
            CliEvent::Init(init) => { /* TODO: Implement init event handling */ },
            CliEvent::List(list) => { /* TODO: Implement list event handling */ },
        }
    }
}

pub fn run(cli: Cli) {
    let opts = Opts { config: cli.config.clone() };
    let cx = Context::new(opts);
    let executor = ExecutorImpl::new();
    let evloop = EventLoop::new(executor);
    let mut rt = Runtime::new(cx, evloop);
    rt.start();
}
