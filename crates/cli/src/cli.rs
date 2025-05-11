use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// rscpio is GNU cpio but extra features
///
/// copies files to and from archives... but advanced
///
/// NOTE: Supports only newc and crc cpio
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Strips the cpio of select information
    Strip(StripArgs),
}

#[derive(Debug, Clone, Args)]
pub struct StripArgs {
    pub input: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub reset_ino: bool,
    #[arg(long)]
    pub reset_mtime: bool,
}

pub fn cli() -> Cli {
    Cli::parse()
}
