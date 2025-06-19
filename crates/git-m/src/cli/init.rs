use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(about = "init")]
pub struct Init {
    #[command(subcommand)]
    pub event: InitEvent,
}

#[derive(Debug, Subcommand)]
pub enum InitEvent {
    Sync(InitSync),
}

#[derive(Debug, Args)]
#[command(about = "sync")]
pub struct InitSync {}
