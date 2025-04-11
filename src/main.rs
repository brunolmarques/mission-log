//! Main entry point of the application.
//! High-level coordinator for CLI argument handling and log analysis.

use log::info;

mod analyzer;
mod cli;
mod parser;

fn main() {
    // Initialize the logger for better debugging/tracing
    env_logger::init();
    info!("Starting space mission log analysis");

    // Parse CLI arguments
    let args = cli::parse_args();

    // Orchestrate analysis
    match analyzer::find_longest_successful_mars_mission(&args.file_path) {
        Ok(security_code) => {
            println!("{}", security_code);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    info!("Analysis complete");
}
