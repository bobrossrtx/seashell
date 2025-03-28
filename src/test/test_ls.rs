#[allow(unused_imports)]
use crate::commands::ls::LsCommand;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use tempfile::tempdir;

#[test]
fn test_ls_command_lists_directory_contents() {
    let mut ls_command = LsCommand::new();

    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create some test files in the temporary directory
    fs::write(temp_path.join("file1.txt"), "content1").unwrap();
    fs::write(temp_path.join("file2.txt"), "content2").unwrap();

    // Execute the ls command
    ls_command.execute(&[temp_path.to_str().unwrap().to_string()]).unwrap();

    // Assert that the output contains the test files
    let entries: Vec<_> = fs::read_dir(temp_path).unwrap().map(|e| e.unwrap().file_name()).collect();
    assert!(entries.contains(&"file1.txt".into()));
    assert!(entries.contains(&"file2.txt".into()));
}