# Seashell

Seashell is a custom shell written in Rust. It provides basic shell functionality, including built-in commands, environment variable management, and external command execution.

## Features
- Built-in commands: `cd`, `ls`, `pwd`, `env`, `set`, `get`, `unset`
- Environment variable management
- Customizable prompt
- Logging for debugging
- Modular and extensible design

## Project Structure
```
seashell
├── src
│   ├── main.rs       # Entry point of the application
│   ├── commands      # Built-in commands
│   │   ├── cd.rs     # Change directory command
│   │   ├── ls.rs     # List directory contents
│   │   ├── pwd.rs    # Print working directory
│   │   └── env       # Environment variable commands
│   │       ├── list.rs
│   │       ├── set.rs
│   │       ├── get.rs
│   │       └── unset.rs
│   ├── utils         # Utility functions and modules
│   │   ├── environment.rs # Global environment variable management
│   │   ├── logger.rs       # Logging utility
│   │   └── seashellprompt.rs # Customizable prompt
├── Cargo.toml        # Cargo configuration file
└── README.md         # Project documentation
```

## Getting Started

### Prerequisites
- Rust and Cargo installed on your machine. Follow the instructions on the [official Rust website](https://www.rust-lang.org/) to set up your environment.

### Running the Application
Navigate to the project directory and use the following command:
```
cargo run
```

### Building the Project
To build the project, use:
```
cargo build
```

### Testing
To run tests for the library code, use:
```
cargo test
```

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to improve Seashell.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.