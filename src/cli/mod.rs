//! CLI argument definition and parsing.

use clap::Parser;

/// Command line arguments for the space mission log analysis.
#[derive(Debug, Parser)]
#[command(name = "space_mission_log_analysis")]
#[command(about = "Analyze space mission logs to find the longest successful Mars mission.")]
pub struct CliArgs {
    /// Path to the space missions log file
    #[arg(short = 'f', long = "file")]
    pub file_path: String,
}

/// Parse the CLI arguments into `CliArgs`.
pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
