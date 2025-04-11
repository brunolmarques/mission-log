//! Parser for lines in the space mission log.

use log::debug;

/// Represents a space mission with the minimal fields we care about.
#[derive(Debug, PartialEq)]
pub struct Mission {
    pub destination: String,
    pub status: String,
    pub duration: u64,
    pub security_code: String,
}

impl Mission {
    /// Creates a new `Mission` from a valid line split.
    /// The `line_split` must contain at least 8 fields in this order:
    ///  0 - Date
    ///  1 - Mission ID
    ///  2 - Destination
    ///  3 - Status
    ///  4 - Crew Size
    ///  5 - Duration (days)
    ///  6 - Success Rate
    ///  7 - Security Code
    pub fn from_line_split(line_split: &[&str]) -> Option<Self> {
        if line_split.len() < 8 {
            return None;
        }

        let destination = line_split[2].trim().to_string();
        debug!("Parsed destination: {}", destination);
        let status = line_split[3].trim().to_string();
        debug!("Parsed status: {}", status);
        let duration_str = line_split[5].trim();
        debug!("Parsed duration string: {}", duration_str);
        let security_code = line_split[7].trim().to_string();
        debug!("Parsed security code: {}", security_code);

        // Parse duration
        debug!("Parsing duration: {}", duration_str);
        let duration = duration_str.parse::<u64>().ok()?;

        Some(Mission {
            destination,
            status,
            duration,
            security_code,
        })
    }
}

/// Parse a single line from the log file into an optional `Mission`.
/// Returns `None` if the line is invalid, commented out, or cannot be parsed.
pub fn parse_line(line: &str) -> Option<Mission> {
    let trimmed = line.trim();

    // Ignore empty lines or commented lines
    if trimmed.is_empty() || trimmed.starts_with('#') {
        debug!("Ignoring comment/empty line: {}", trimmed);
        return None;
    }

    // Split by '|' which may have inconsistent spacing
    let parts: Vec<&str> = trimmed.split('|').map(|s| s.trim()).collect();
    Mission::from_line_split(&parts)
}

//---------------------------------------------------Tests---------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_valid() {
        let line = "2045-07-12 | KLM-1234 | Mars | Completed | 5 | 387 | 98.7 | TRX-842-YHG";
        let mission = parse_line(line).unwrap();
        assert_eq!(mission.destination, "Mars");
        assert_eq!(mission.status, "Completed");
        assert_eq!(mission.duration, 387);
        assert_eq!(mission.security_code, "TRX-842-YHG");
    }

    #[test]
    fn test_parse_line_commented() {
        let line = "# 2045-07-12 | KLM-1234 | Mars | Completed | 5 | 387 | 98.7 | TRX-842-YHG";
        assert_eq!(parse_line(line), None);
    }

    #[test]
    fn test_parse_line_incomplete() {
        let line = "2045-07-12 | KLM-1234 | Mars";
        assert_eq!(parse_line(line), None);
    }
}
