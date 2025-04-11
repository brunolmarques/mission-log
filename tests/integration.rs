//! Integration test for the entire application.

use assert_cmd::Command;
use std::fs::File;
use std::io::Write;

/// This is an end-to-end test that checks if the executable
/// prints the expected result for a crafted log file.
#[test]
fn test_integration_longest_mars_mission() {
    let input = r#"
# Some comment
2030-11-20 | ABC-1234 | Mars | Completed | 10 | 250 | 96.0 | SEC-XYZ-111
2031-01-10 | ZZZ-4321 | Mars | Completed | 8 | 400 | 99.9 | SEC-ABC-999
2031-05-12 | TTT-9999 | Jupiter | Completed | 6 | 500 | 87.0 | SEC-JUP-500
    "#;

    // Write the input to a temporary file
    let file_path = "integration_test.log";
    let mut file = File::create(file_path).unwrap();
    file.write_all(input.as_bytes()).unwrap();

    // Run the built binary using assert_cmd
    let mut cmd = Command::cargo_bin("space_mission_log_analysis").unwrap();
    let assert = cmd.arg("--file").arg(file_path).assert();

    assert.success().stdout("SEC-ABC-999\n");

    // Clean up
    std::fs::remove_file(file_path).unwrap();
}
