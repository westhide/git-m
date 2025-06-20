use git_m::{
    cli::Cli,
    error::Result,
    log::{debug, init_tracing_subscriber_log},
    runner::run,
};
use nill::{Nil, nil};

#[tokio::main]
pub async fn main() -> Result<Nil> {
    init_tracing_subscriber_log();

    debug!("Git Mono");
    let cli = Cli::default();

    run(cli).await?;

    Ok(nil)
}
