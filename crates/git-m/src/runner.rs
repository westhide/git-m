use std::sync::Arc;

use futures::StreamExt;
use nill::{Nil, nil};

use crate::{
    cli::{Cli, Event},
    config::{Config, gdir::Gdir, hub::Hub},
    error::Result,
    event::{event_loop::EventLoop, execute::Execute},
    fs::walkdir::WalkDir,
    git::{Git, IGit},
    log::{debug, info, instrument},
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
    type Return = Result<Nil>;

    #[instrument(skip(self))]
    async fn execute(&mut self, event: Self::Event) -> Self::Return {
        debug!(?self.ctx);

        match event {
            Event::Init(init) => {
                todo!()
            },
            Event::List(list) => {
                let mut walkdir = WalkDir::new(&list.path, Git::open);
                let mut hub = Hub::new(&list.path);
                while let Some(gdir) = walkdir.next().await {
                    let g: Gdir = gdir?.into();
                    hub.push(g);
                }
                let mut config = Config::from(hub);
                debug!(?config);
                let toml = config.encode()?;
                info!(toml);
                println!("{toml}");
            },
        }

        Ok(nil)
    }
}

pub async fn run(cli: Cli) -> Result<Nil> {
    let opts = Opts { config: cli.config.clone() };
    let ctx = Context::new(opts);
    let rt = Runtime::new(ctx);

    let mut handler = EventLoop::startup(Executor::new(rt.ctx.clone()));
    handler.push(cli.event).await;
    handler.into_handle().await??;

    Ok(nil)
}
