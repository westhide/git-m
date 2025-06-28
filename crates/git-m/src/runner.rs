use std::sync::Arc;

use futures::StreamExt;
use nill::{Nil, nil};

use crate::{
    cli::{Cli, Event},
    error::{Error, Result},
    event::{event_loop::EventLoop, execute::Execute},
    fs::walkdir::WalkDir,
    git::{Git, gnu::Gnu},
    log::{debug, instrument},
    runtime::{
        Runtime,
        context::{Context, opts::Opts},
    },
};

#[derive(Debug, Clone)]
pub struct Executor {
    ctx: Arc<Context>,
}

impl Executor {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx }
    }
}

unsafe impl Send for Event {}

impl Execute for Executor {
    type Event = Event;
    type Return = Result<Nil, Error>;

    #[instrument(skip(self))]
    async fn execute(&mut self, event: Self::Event) -> Self::Return {
        debug!(?self.ctx);

        match event {
            Event::Init(init) => {
                todo!()
            },
            Event::List(list) => {
                let mut walkdir = WalkDir::new(list.path, |p| Gnu::is_git_repo(p));
                while let Some(repo) = walkdir.next().await {
                    let repo = repo?;
                    // debug!(?repo)
                }
            },
        }

        Ok(nil)
    }
}

pub async fn run(cli: Cli) -> Result<Nil> {
    let opts = Opts { config: cli.config.clone() };
    let ctx = Context::new(opts);
    let evloop = EventLoop::new();
    let mut rt = Runtime::new(ctx, evloop);

    rt.evloop.startup(Executor::new(rt.ctx.clone()));
    rt.evloop.push(cli.event).await?;
    rt.evloop.release();

    if let Some(handle) = rt.evloop.handle {
        handle.await??;
    }

    Ok(nil)
}
