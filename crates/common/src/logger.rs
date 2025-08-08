use tracing::Level;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging(crate_name: &str) {
    Builder::new()
        .with_crate_name(crate_name)
        .with_crate_level(Level::TRACE)
        .init();
}

/// Build a logger
///
/// ```
/// Builder::new()
///     .with_default_level(Level::ERROR)
///     .with_crate_name(env!("CARGO_CRATE_NAME"))
///     .with_crate_level(Level::INFO)
///     .with_file(true)
///     .with_line_number(true)
///     .init();
/// ```
#[derive(Debug)]
pub struct Builder {
    targets: Targets,
    default_level: Level,
    crate_name: Option<String>,
    crate_level: Level,
    show_filename: bool,
    show_line_number: bool,
}

impl Builder {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            targets: Targets::new(),
            default_level: Level::ERROR,
            crate_name: None,
            crate_level: Level::INFO,
            show_filename: false,
            show_line_number: false,
        }
    }
    #[must_use]
    pub fn with_default_level(mut self, level: Level) -> Self {
        self.default_level = level;
        self
    }
    #[must_use]
    pub fn with_crate_name(mut self, name: &str) -> Self {
        self.crate_name = Some(name.to_string());
        self
    }
    #[must_use]
    pub fn with_crate_level(mut self, level: Level) -> Self {
        self.crate_level = level;
        self
    }
    #[must_use]
    pub fn with_file(mut self, show: bool) -> Self {
        self.show_filename = show;
        self
    }
    #[must_use]
    pub fn with_line_number(mut self, show: bool) -> Self {
        self.show_line_number = show;
        self
    }
    pub fn init(self) {
        let mut layer = self.targets.with_default(self.default_level);
        if let Some(crate_name) = self.crate_name {
            layer = layer.with_target(crate_name, self.crate_level);
        }
        tracing_subscriber::registry()
            .with(layer)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_file(self.show_filename)
                    .with_line_number(self.show_line_number),
            )
            .init();
    }
}
