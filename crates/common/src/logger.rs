use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging(crate_name: &str) {
    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(Level::DEBUG)
        .with_target(crate_name, tracing::Level::TRACE);
    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                // .with_file(true)
                // .with_line_number(true),
        )
        .init();
}
