use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about, version, propagate_version = true)]
pub struct Opts {
    #[arg(short, long, default_value = "gm.toml", help = "Config file")]
    pub config: String,

    #[command(subcommand)]
    pub action: Action,
}

impl Default for Opts {
    fn default() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Init(InitOpts),
    List(ListOpts),
}

#[derive(Debug, Args)]
#[command(about = "init")]
pub struct InitOpts {
    #[clap(subcommand)]
    pub action: InitAction,
}

#[derive(Debug, Subcommand)]
pub enum InitAction {
    Sync(InitSyncOpts),
}

#[derive(Debug, Args)]
#[command(about = "sync")]
pub struct InitSyncOpts {}

#[derive(Debug, Args)]
#[command(about = "list")]
pub struct ListOpts {}
