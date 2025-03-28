// use crate::commands::env::EnvSetCommand;
// use crate::commands::env::EnvUnsetCommand;
// use crate::commands::env::EnvListCommand;
// use std::env;

// #[test]
// fn test_env_set_command_sets_variable() {
//     let mut set_command = EnvSetCommand::new();

//     // Set an environment variable
//     set_command.execute(&["TEST_KEY".to_string(), "TEST_VALUE".to_string()]).unwrap();

//     // Assert that the variable is set
//     assert_eq!(env::var("TEST_KEY").unwrap(), "TEST_VALUE");
// }

// #[test]
// fn test_env_unset_command_unsets_variable() {
//     let mut unset_command = EnvUnsetCommand::new();

//     // Set an environment variable
//     env::set_var("TEST_KEY", "TEST_VALUE");

//     // Unset the environment variable
//     unset_command.execute(&["TEST_KEY".to_string()]).unwrap();

//     // Assert that the variable is unset
//     assert!(env::var("TEST_KEY").is_err());
// }

// #[test]
// fn test_env_list_command_lists_variables() {
//     let mut list_command = EnvListCommand::new();

//     // Set some environment variables
//     env::set_var("TEST_KEY1", "VALUE1");
//     env::set_var("TEST_KEY2", "VALUE2");

//     // Execute the list command
//     list_command.execute(&[]).unwrap();

//     // Assert that the variables are listed (this would require capturing stdout in a real test)
//     assert!(env::var("TEST_KEY1").is_ok());
//     assert!(env::var("TEST_KEY2").is_ok());
// }