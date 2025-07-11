#!/bin/bash

# Check if server IP is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <server_ip>"
    echo "Example: $0 192.168.1.100"
    exit 1
fi

SERVER_IP=$1

# Build in debug mode
echo "Building StickDeck Linux client in debug mode..."
cd .. && cargo build
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Check if user can access /dev/uinput
if [ ! -w "/dev/uinput" ]; then
    echo "Warning: Cannot write to /dev/uinput. You may need to:"
    echo "  1. Run with sudo: sudo $0 $SERVER_IP"
    echo "  2. Or add your user to the 'input' group: sudo usermod -aG input $USER"
    echo "     (logout and login again for this to take effect)"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo "Starting StickDeck Linux client in debug mode..."
echo "Connecting to server at $SERVER_IP"
echo ""
RUST_LOG=debug ./target/debug/stickdeck-linux "$SERVER_IP"