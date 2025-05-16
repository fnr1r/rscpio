use camino::Utf8PathBuf as PathBuf;
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
    #[command(flatten)]
    pub args: SharedArgs,
}

#[derive(Debug, Clone, Args)]
pub struct SharedArgs {
    /// Change to directory DIR
    #[arg(short = 'D', long)]
    pub directory: Option<PathBuf>,
    /// Verbosely list the files processed
    #[arg(short, long)]
    pub verbose: bool,
    /// Print a "." for each file processed
    #[arg(short = 'V', long = "dot")]
    pub print_dot: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// List files
    List(ListArgs),
    /// Strips the cpio of select information
    Strip(StripArgs),
}

#[derive(Debug, Clone, Args)]
pub struct ListArgs {
    pub input: Option<PathBuf>,
    /// end each output line with NUL, not newline
    #[arg(long)]
    pub zero: bool,
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
    #[arg(long)]
    pub sort: bool,
}

pub fn cli() -> Cli {
    Cli::parse()
}
