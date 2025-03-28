#[allow(unused_imports)]
use crate::commands::seashell::seashell_version;

#[test]
fn test_seashell_version_command_executes() {
    let mut version_command = seashell_version::SeashellVersionCommand::new();

    // Execute the seashell_version command
    let result = version_command.execute(&[]);

    // Assert that the command executes successfully
    assert!(result.is_ok());
}