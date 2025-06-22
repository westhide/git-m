pub use tracing::*;

pub fn init_tracing_subscriber_log() {
    use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
    let fenv = EnvFilter::from_default_env();
    let kind = FmtSpan::NEW | FmtSpan::CLOSE;
    tracing_subscriber::fmt().with_env_filter(fenv).with_span_events(kind).init();
}
