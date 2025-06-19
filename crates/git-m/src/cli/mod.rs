pub mod init;
pub mod list;

use clap::{Parser, Subcommand};

use crate::{
    cli::{init::Init, list::List},
    constant::CONFIG_FILE_NAME,
};

#[derive(Debug, Parser)]
#[command(about, version, propagate_version = true)]
pub struct Cli {
    #[arg(long, default_value = CONFIG_FILE_NAME, help = "config file")]
    pub config: String,

    #[command(subcommand)]
    pub event: Event,
}

#[derive(Debug, Subcommand)]
pub enum Event {
    Init(Init),
    List(List),
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}
