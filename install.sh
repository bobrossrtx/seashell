#!/bin/bash

# Add Cargo to PATH for sudo
export PATH="/home/bobrossrtx/.cargo/bin:$PATH"

# Define installation paths
INSTALL_DIR="/usr/local/bin"
SEASHELL_BINARY="seashell"

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# Ensure the script is run with sudo
if [ "$EUID" -ne 0 ]; then
  echo -e "${RED}Please run as root (e.g., sudo ./install.sh)${NC}"
  exit 1
fi

# Check if Rust is installed
echo -e "${BLUE}Checking if Rust is installed...${NC}"
if ! command -v rustc >/dev/null 2>&1; then
  echo -e "${YELLOW}Rust is not installed. Installing Rust...${NC}"
  # Prompt for installation
  read -p "Do you want to install Rust? (y/n): " install_rust
  if [[ "$install_rust" != "y" && "$install_rust" != "Y" ]]; then
    echo -e "${RED}Rust installation skipped. Exiting.${NC}"
    exit 1
  fi

  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
fi
echo -e "${GREEN}Rust is installed.${NC}\n"

# Set the Rust toolchain to stable
echo -e "${BLUE}Setting Rust toolchain to stable...${NC}"
if ! rustup default stable; then
  echo -e "${RED}Failed to set Rust toolchain to stable. Check your Rust installation.${NC}"
  exit 1
fi
echo -e "${GREEN}Rust toolchain set to stable.${NC}\n"

# Build the seashell binary
echo -e "${BLUE}Building the seashell binary...${NC}"
if ! cargo build --release; then
  echo -e "${RED}Failed to build seashell. Ensure Rust and Cargo are installed.${NC}"
  exit 1
fi
echo -e "${GREEN}Seashell binary built successfully.${NC}\n"

# Copy the binary to the installation directory
echo -e "${BLUE}Installing seashell to ${INSTALL_DIR}...${NC}"
if cp target/release/$SEASHELL_BINARY $INSTALL_DIR; then
  echo -e "${GREEN}Seashell installed successfully to ${INSTALL_DIR}/${SEASHELL_BINARY}${NC}"
else
  echo -e "${RED}Failed to copy seashell to ${INSTALL_DIR}. Check permissions.${NC}"
  exit 1
fi
echo -e "${GREEN}Installation completed.${NC}\n"

# Verify installation
echo -e "${BLUE}Verifying installation...${NC}"
if command -v $SEASHELL_BINARY >/dev/null 2>&1; then
  echo -e "${GREEN}Seashell is now available as a system command.${NC}"
else
  echo -e "${YELLOW}Installation completed, but seashell is not in the PATH.${NC}"
fi
echo -e "${GREEN}You can run seashell by typing: ${INSTALL_DIR}/${SEASHELL_BINARY}${NC}"
echo -e "${GREEN}or ${SEASHELL_BINARY} if it's in your PATH.${NC}\n"


# Warn about setting default shell
echo -e "${YELLOW}WARNING: seashell is not ready to be set as your default shell yet.${NC}"
echo -e "${YELLOW}It is still in development and may not work as expected, such as tonnes of compatibility issues.${NC}"
echo -e "${YELLOW}Please check the README for more information.${NC}"
echo -e "${YELLOW}You can set seashell as your default shell by running: (RUN THIS AT YOUR OWN RISK!, IT MAY BREAK YOUR SHELL)${NC}"
echo -e "${RED}chsh -s ${INSTALL_DIR}/${SEASHELL_BINARY}${NC}"
