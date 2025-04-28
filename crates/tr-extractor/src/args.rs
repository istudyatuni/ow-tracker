use std::path::PathBuf;

use clap::Parser;

/// Translation extractor for Outer Wilds
#[derive(Debug, Parser)]
pub struct Cli {
    /// Path to game's data directory
    #[arg(long)]
    pub data_dir: Option<PathBuf>,

    /// Write files
    #[arg(long)]
    pub write: bool,

    /// Verbose logging
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,
}
