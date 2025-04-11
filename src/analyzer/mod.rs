//! Contains logic to find the longest successful Mars mission.
//! Uses Rayon for parallel processing of large files.

use std::fs;
use rayon::prelude::*;

use crate::parser::{parse_line, Mission};
use log::{info, warn};

/// Reads the file at `file_path`, parses each line, and returns the security code
/// of the longest successful Mars mission.
///
/// # Errors
/// Returns an error if the file cannot be read or if no valid mission is found.
pub fn find_longest_successful_mars_mission(file_path: &str) -> Result<String, String> {
    info!("Reading file: {}", file_path);

    let content = fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {e}"))?;
    info!("File loaded, processing lines...");

    // Process lines in parallel using rayon
    let missions: Vec<Mission> = content
        .par_lines()
        .filter_map(|line| parse_line(line))
        .filter(|m| m.status.eq("Completed") && m.destination.eq("Mars"))
        .collect();

    if missions.is_empty() {
        warn!("No valid successful Mars missions found!");
        return Err("No successful Mars missions found in the file.".into());
    }

    // Find mission with the longest duration
    let longest_mission = missions
        .into_iter()
        .max_by_key(|m| m.duration)
        .expect("missions vector should not be empty");

    Ok(longest_mission.security_code)
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
        2045-07-12 | KLM-1234 | Mars  | Completed | 5 | 100 | 98.7 | CODE-ABC\n\
        2046-07-12 | XYZ-9999 | Mars  | Completed | 5 | 200 | 99.0 | BIGGER-999\n\
        2047-01-01 | ABC-7777 | Venus | Completed | 3 |  10 | 90.0 | IGNORE-ME\n\
        ";
        let tmp_file = "test_data.log";
        std::fs::write(tmp_file, input).unwrap();

        let result = find_longest_successful_mars_mission(tmp_file).unwrap();
        assert_eq!(result, "BIGGER-999");

        std::fs::remove_file(tmp_file).unwrap();
    }
}
