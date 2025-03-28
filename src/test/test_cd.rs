#[allow(unused_imports)]
use crate::commands::cd::CdCommand;
#[allow(unused_imports)]
use std::env;

#[test]
fn test_cd_command_changes_directory() {
    let mut cd_command = CdCommand::new();

    // Save the current directory
    let original_dir = env::current_dir().unwrap();

    // Create a temporary directory for testing
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap().to_string();

    // Execute the cd command to change to the temporary directory
    cd_command.execute(&[temp_path.clone()]).unwrap();

    // Assert that the current directory has changed
    let current_dir = env::current_dir().unwrap();
    assert_eq!(current_dir.to_str().unwrap(), temp_path);

    // Change back to the original directory
    cd_command.execute(&[original_dir.to_str().unwrap().to_string()]).unwrap();

    // Assert that the current directory is restored
    let restored_dir = env::current_dir().unwrap();
    assert_eq!(restored_dir, original_dir);
}