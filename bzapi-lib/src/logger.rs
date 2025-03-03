use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(level: Level) {
    // Initialize tracing with colored output and environment filter
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_span_events(FmtSpan::FULL)
                .with_ansi(true)
                .without_time(),
        )
        .with(LevelFilter::from(level))
       /*  .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        ) */
        .init();
}
