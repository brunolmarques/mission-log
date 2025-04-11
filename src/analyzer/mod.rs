//! Contains logic to find the longest successful Mars mission.
//! Uses Rayon for parallel processing of large files.

use rayon::prelude::*;
use std::fs;

use crate::parser::{Mission, parse_line};
use log::{debug, info, warn};

/// Reads the file at `file_path`, parses each line, and returns the security code
/// of the longest successful Mars mission.
///
/// # Errors
/// Returns an error if the file cannot be read or if no valid mission is found.
pub fn find_longest_successful_mars_mission(file_path: &str) -> Result<Vec<Mission>, String> {
    info!("Reading file: {}", file_path);

    let content = fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {e}"))?;
    info!("File loaded, processing lines...");

    // Process lines in parallel using rayon
    let missions: Vec<Mission> = content
        .par_lines()
        .filter_map(parse_line)
        .filter(|m| m.status.eq("Completed") && m.destination.eq("Mars"))
        .collect();

    if missions.is_empty() {
        warn!("No valid successful Mars missions found!");
        return Err("No successful Mars missions found in the file.".into());
    }

    // Determine the maximum duration out of all missions.
    // Since we already know `missions` is non-empty, `max()` is safe to unwrap.
    info!(
        "Found {} successful Mars missions, searching for the longest...",
        missions.len()
    );
    debug!("Missions: {:?}", missions);
    let max_duration = missions.iter().map(|m| m.duration).max().unwrap();

    // Collect all missions that share this maximum duration.
    let longest_missions: Vec<Mission> = missions
        .into_iter()
        .filter(|m| m.duration == max_duration)
        .collect();

    Ok(longest_missions)
}

//---------------------------------------------------Tests---------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_longest_successful_mars_mission_no_data() {
        // Input with no valid lines
        let input = "# This is a comment\n# Another comment\n";
        let tmp_file = "test_empty.log";
        std::fs::write(tmp_file, input).unwrap();

        let result = find_longest_successful_mars_mission(tmp_file);
        assert!(result.is_err());

        std::fs::remove_file(tmp_file).unwrap();
    }

    #[test]
    fn test_find_longest_successful_mars_mission_ok() {
        let input = "\
        # comment\n\
        2045-07-12 | KLM-1234 | Mars  | Completed | 5 | 200 | 98.7 | CODE-ABC\n\
        2046-07-12 | XYZ-9999 | Mars  | Completed | 5 | 200 | 99.0 | BIGGER-999\n\
        2047-01-01 | ABC-7777 | Venus | Completed | 3 |  10 | 90.0 | IGNORE-ME\n\
        ";
        let tmp_file = "test_data.log";
        std::fs::write(tmp_file, input).unwrap();

        let result = find_longest_successful_mars_mission(tmp_file).unwrap();
        // Since both valid Mars missions have the same duration, we expect both to be returned.
        assert_eq!(result.len(), 2);
        let security_codes: Vec<_> = result.iter().map(|m| m.security_code.as_str()).collect();
        assert!(security_codes.contains(&"CODE-ABC"));
        assert!(security_codes.contains(&"BIGGER-999"));

        std::fs::remove_file(tmp_file).unwrap();
    }
}
