use clap::Parser;

#[derive(Parser)]
#[clap(
    name = clap::crate_name!(),
    about = clap::crate_description!(),
    long_version = clap::crate_version!(),
    propagate_version = true,
)]
pub struct Opts {
    #[clap(short, long, default_value = "gm.toml", help = "Config file")]
    pub config: String,

    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Parser)]
pub enum Action {
    Init(Init),
}

#[derive(Parser)]
#[clap(about = "Init")]
pub struct Init {
    #[clap(subcommand)]
    pub action: InitAction,
}

#[derive(Parser)]
pub enum InitAction {
    Sync(InitSync),
}

#[derive(Parser)]
#[clap(about = "Sync")]
pub struct InitSync {
    #[clap(short, long, default_value = "gm.toml", help = "Config file")]
    pub config: String,
}

pub fn parse() -> Opts {
    Opts::parse()
}
