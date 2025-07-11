# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Stickdeck-RS transforms a Steam Deck into a virtual game controller for PC using a client-server architecture over TCP.

## Key Architecture

### Components
- **common/**: Shared networking protocol and data structures (Packet, Mouse, MouseButton, XGamepad, XButtons)
- **deck/**: Server running on Steam Deck - captures inputs via Steam Input API
- **win/**: Windows client - creates virtual Xbox 360 controller via ViGEm
- **linux/**: Linux client - creates virtual Xbox 360 controller via uinput

### Network Protocol
- TCP on port 7777
- 16-byte binary packets
- Packet types: Timestamp, Gamepad, Mouse
- Only sends on input changes

### Threading Model
- Server: Separate threads for GUI (Iced), input polling, and network
- Client: Main thread for controller, network thread for receiving

## Essential Commands

### Building
```bash
# Server (Steam Deck)
cd deck && cargo build --release

# Windows Client  
cd win && cargo build --release

# Linux Client
cd linux && cargo build --release

# Debug builds
cd deck && cargo build
cd win && cargo build
cd linux && cargo build
```

### Testing
```bash
# Run tests for a specific component
cd deck && cargo test
cd win && cargo test
cd linux && cargo test
cd common && cargo test

# Run specific test
cd deck && cargo test test_name
```

### Running
```bash
# Server (use provided scripts)
cd deck && ./launch.sh    # Release mode
cd deck && ./debug.sh     # Debug mode with logging

# Windows Client
cd win && cargo run --release -- <server_ip>

# Linux Client
cd linux && cargo run --release -- <server_ip>
# Or use provided scripts
cd linux && ./scripts/launch.sh <server_ip>
cd linux && ./scripts/debug.sh <server_ip>
```

### Development
```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Check types
cargo check
```

## Code Patterns

### Error Handling
- Use `Result<T, Box<dyn Error>>` for main functions
- Use `anyhow::Result` or specific error types in libraries
- Always handle disconnections gracefully

### Performance Considerations
- Use bounded channels (capacity ~10) to prevent buffer growth
- Avoid allocations in hot paths (input polling)
- Use `perf!` macro for performance monitoring in debug builds

### Logging
- Default log level: info
- Debug level shows update rates and performance metrics
- Use `RUST_LOG=debug` environment variable

## Key Implementation Details

### Steam Deck Server
- Requires Steam client running
- Uses Steamworks SDK 154 (AppID 480 - Spacewar)
- Input polling at configurable rate (default 3ms)
- GUI built with Iced framework

### Windows Client  
- Requires ViGEm Bus Driver installed
- Emulates Xbox 360 controller
- Mouse support via SendInput API
- Auto-reconnect on disconnection

### Linux Client
- Requires /dev/uinput access (sudo or input group membership)
- Uses input-linux crate for uinput integration
- Emulates Xbox 360 controller compatible with evdev
- Mouse support via uinput mouse device
- Auto-reconnect on disconnection

### Packet Structure
- Fixed 16-byte frames
- Binary serialization (not JSON)
- See common/src/lib.rs for Packet enum

## Testing Virtual Controller
- Windows: Use `joy.cpl` (Game Controllers panel)
- Linux: Use `evtest` or `jstest-gtk` to test the virtual controller
- Steam Deck: Configure inputs in Steam's controller settings

## Release Process
- Version synchronized across deck/Cargo.toml, win/Cargo.toml, and linux/Cargo.toml
- GitHub Actions builds release binaries for all platforms
- Update CHANGELOG.md with changes