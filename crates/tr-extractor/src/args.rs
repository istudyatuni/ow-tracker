use clap::Parser;

/// Translation extractor for Outer Wilds
#[derive(Debug, Parser)]
pub struct Cli {
    /// Write files
    #[arg(long)]
    pub write: bool,
}
