#!/bin/bash

echo -e "This script will install cpr compiler on your system."
read -p "Do you want to proceed with the installation? (Y/n) " choice

# Colors Variables
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NONE='\033[0m' # No Color

if [[ "$choice" =~ ^[Yy]$ ]]; then
  if [[ $EUID -ne 0 ]]; then
    echo -e "${RED}[ERROR]${NONE}: This script must be run as root."
    read -p "Do you want to login as root? (Y/n) " loginChoice
    if [[ "$loginChoice" =~ ^[Yy]$ ]]; then
      echo -e "${YELLOW}[WARNING]${NONE}: You have to run the install script again."
      su root
    else
      echo -e "${RED}[ERROR]${NONE}: Installation cancelled. Please login as root to continue the installation."
      exit 1
    fi
  fi

  echo -e "Checking if Rust is installed..."
  if ! command -v rustc &> /dev/null; then
    echo -e "${RED}[ERROR]${NONE}: Rust is not installed. You can install by saying `y` below or install rust from their original website."
    read -p "Do you want to proceed with the installation? (Y/n) " choiceInstall
    if [[ "$choiceInstall" =~ ^[Yy]$ ]]; then
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      rustup default stable
    else
      echo -e "${RED}[ERROR]${NONE}: Installation cancelled. Please install Rust from their original website."
      exit 1
    fi
  fi
  rustup default stable

  # Make sure bin is exist
  echo -e "Creating a bin directory in /usr/local/bin..."
  mkdir -p /usr/local/bin

  # Make sure the build part is in bin
  cd /usr/local/bin/

  # Download the file
  echo -e "Downloading cpr.rs from Github..."
  curl -LJO https://github.com/codernocook/cpr_lang/releases/download/alpha/cpr.rs

  echo -e "Compiling cpr.rs..."
  # rustc --opt-level=3 --out-dir=/usr/local/bin cpr.rs
  rustc cpr.rs

  echo -e "Making cpr executable..."
  chmod +x /usr/local/bin/cpr

  # Make cpr can be run anywhere in console
  export PATH=$PATH:/usr/local/bin/cpr

  echo -e "${GREEN}[SUCCESS]${NONE}: Installation complete."
else
  echo -e "${RED}[ERROR]${NONE}: Installation cancelled."
fi
