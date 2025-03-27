# Seashell Roadmap

## Current Features
- Basic user input handling with a prompt.
- Exit functionality with `exit` or `quit` commands.
- Command parsing into name and arguments.
- Built-in commands: `cd`, `ls`, `pwd`.
- External command execution using `std::process::Command`.
- Environment variable support: setting, getting, listing, and unsetting variables.
- Customizable prompt with support for colors and environment variables.
- Logging utility for debugging purposes, storing logs in `~/.seashell/seashell.log`.

## Updated Project Structure
- Organized the project into modular directories:
  - `src/commands/` for built-in commands.
  - `src/utils/` for utility functions.

## Completed
- Refactored environment variable commands (`env`, `set`, `get`, `unset`) into individual commands.
- Centralized environment variable management in `environment.rs` for global access.
- Improved project structure by organizing `env` commands into `src/commands/env/`.
- Enhanced help screens with better formatting and dynamic flag/option descriptions.

## Next Steps
1. **Command History**
   - Implement a history feature to recall previous commands.

2. **Scripting Support**
   - Add support for running scripts written in Seashell.

3. **Error Handling**
   - Improve error messages and handling for invalid inputs or commands.

4. **Advanced Prompt Features**
   - Add more customization options for the prompt, such as dynamic elements.

5. **Macros & Aliases**
   - **Aliases**: Allow users to define shortcuts for commands or sequences of commands.
   - **Macros**: Enable users to create reusable command sequences with parameters.

6. **Testing Framework**
   - Implement a testing framework for unit and integration tests.
   - Write tests for existing commands and features.

7. **Documentation**
   - Create comprehensive documentation for Seashell, including usage examples and command references.
   - Write a README file with installation instructions and a quick start guide.
- Add a `CHANGELOG.md` to track changes and updates.
- Add a `CONTRIBUTING.md` file to guide contributors on how to contribute to the project.
- Add a `LICENSE` file to clarify the project's licensing terms.
- Add a `CODE_OF_CONDUCT.md` to establish guidelines for community behavior.

## Long-term Goals
- **Plugin System**: Allow users to extend Seashell with plugins.
- **Theming**: Add support for themes and colors.
- **Cross-platform Support**: Ensure compatibility with Windows, macOS, and Linux.
- **Advanced Features**: Add job control, piping, and redirection.