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
    info!("Parsed arguments: {:?}", args);

    // Orchestrate analysis
    match analyzer::find_longest_successful_mars_mission(&args.file_path) {
        Ok(longest_missions) => {
            // All of these missions share the same max duration.
            // We can safely get the duration from the first one since we know it's not empty.
            if longest_missions.len() == 1 {
                println!("{}", longest_missions[0].security_code);
            } else {
                // All missions share the same max duration.
                let longest_duration = longest_missions[0].duration;
                println!(
                    "Found {} longest successful Mars mission(s) lasting {} days:",
                    longest_missions.len(),
                    longest_duration
                );
                for mission in &longest_missions {
                    println!("  Security Code: {}", mission.security_code);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    info!("Analysis complete");
}
