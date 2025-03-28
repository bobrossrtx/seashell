#[allow(unused_imports)]
use crate::commands::pwd::PwdCommand;
#[allow(unused_imports)]
use std::env;

#[test]
fn test_pwd_command_displays_correct_directory() {
    let mut pwd_command = PwdCommand::new();

    // Save the current directory
    let original_dir = env::current_dir().unwrap();

    // Execute the pwd command
    pwd_command.execute(&[]).unwrap();

    // Assert that the output matches the current directory
    let current_dir = env::current_dir().unwrap();
    assert_eq!(current_dir.to_str().unwrap(), original_dir.to_str().unwrap());
}