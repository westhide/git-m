use git_m::{cli::Cli, opts::Opts};

pub fn main() {
    let cli = Cli::default();
    let opts = Opts::from(&cli);
    cli.event
}
