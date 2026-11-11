use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,smart_filee=debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(false).compact())
        .init();
}
