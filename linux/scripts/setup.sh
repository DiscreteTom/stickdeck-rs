#!/bin/bash

echo "StickDeck Linux Client Setup"
echo "============================"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo "Please do not run this script as root/sudo"
    echo "The script will ask for sudo when needed"
    exit 1
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for Rust
echo "Checking for Rust installation..."
if ! command_exists rustc; then
    echo "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
else
    echo "✓ Rust is installed ($(rustc --version))"
fi

# Check for cargo
if ! command_exists cargo; then
    echo "Cargo is not installed. This should come with Rust."
    exit 1
else
    echo "✓ Cargo is installed"
fi

# Check if /dev/uinput exists
echo ""
echo "Checking for uinput support..."
if [ ! -e "/dev/uinput" ]; then
    echo "✗ /dev/uinput not found. Loading uinput module..."
    sudo modprobe uinput
    if [ $? -ne 0 ]; then
        echo "Failed to load uinput module. Your kernel may not support it."
        exit 1
    fi
    echo "✓ uinput module loaded"
else
    echo "✓ /dev/uinput exists"
fi

# Check permissions
echo ""
echo "Checking permissions..."
if [ -w "/dev/uinput" ]; then
    echo "✓ You have write access to /dev/uinput"
else
    echo "✗ You don't have write access to /dev/uinput"
    echo ""
    echo "To fix this, you can either:"
    echo "1. Add your user to the 'input' group (recommended):"
    echo "   sudo usermod -aG input $USER"
    echo "   Then logout and login again"
    echo ""
    echo "2. Run the client with sudo (not recommended for regular use)"
    echo ""
    read -p "Add $USER to input group now? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        sudo usermod -aG input $USER
        echo "✓ Added $USER to input group"
        echo "⚠ You need to logout and login again for this to take effect!"
    fi
fi

# Build the client
echo ""
echo "Building StickDeck Linux client..."
cargo build --release
if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
else
    echo "✗ Build failed. Please check the error messages above."
    exit 1
fi

# Test tools
echo ""
echo "Optional: Installing useful testing tools..."
echo "These tools help test gamepad functionality:"
echo "- evtest: Test input events"
echo "- jstest-gtk: Graphical gamepad tester"
echo ""
read -p "Install testing tools? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if command_exists apt-get; then
        sudo apt-get update && sudo apt-get install -y evtest jstest-gtk
    elif command_exists dnf; then
        sudo dnf install -y evtest jstest-gtk
    elif command_exists pacman; then
        sudo pacman -S evtest jstest-gtk
    else
        echo "Package manager not recognized. Please install evtest and jstest-gtk manually."
    fi
fi

echo ""
echo "Setup complete!"
echo ""
echo "To run StickDeck Linux client:"
echo "  ./launch.sh <steam_deck_ip>"
echo ""
echo "To test your virtual gamepad:"
echo "  evtest  # List and test input devices"
echo "  jstest-gtk  # Graphical gamepad tester"
echo ""
if [ ! -w "/dev/uinput" ]; then
    echo "⚠ Remember to logout and login again if you added yourself to the input group!"
fi