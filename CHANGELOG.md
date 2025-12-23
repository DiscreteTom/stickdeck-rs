# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- CLI framework support with `--help` and `--version` flags (Client & Server)

## [0.3.1] - 2024-10-19

### Added

- Performance warning in debug log level (Client & Server)
- Toggle to disable debug info to prevent memory leak (Server)

### Changed

- Less unnecessary logs in debug mode (Server)

## [0.3.0] - 2024-07-24

### Added

- `launch.bat` to start the client side and create a log file for debugging (Client)
- Retry functionality when connecting to the server (Client)
- Support for left & right mouse buttons and scroll wheel (Client)
- Log file creation for debugging (Server)
- `Left Mouse Button`, `Right Mouse Button` and `Mouse Scroll` to the action set (Server)

### Changed

- **BREAKING**: Updated network protocol and action set
- **BREAKING**: Removed `LeftMouse` and `RightMouse` from the action set, use `MouseMove` instead (Server)

### Fixed

- `debug.bat` will pause on error (Client)

## [0.2.1] - 2024-06-29

### Fixed

- Applied [`SendInput`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput) API instead of [`SetCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos) to move the mouse cursor for better compatibility against games (Client)

## [0.2.0] - 2024-06-29

### Added

- `absolute_mouse` actions will control the mouse cursor directly (Client)

### Changed

- **BREAKING**: Updated network protocol
- Increased vertical padding to fit performance overlay level 2 (Server)
- `debug.sh` now sets `RUST_LOG=debug` (Server)

## [0.1.2] - 2024-06-28

### Added

- Auto build the client side with GitHub Actions (Client)
- Unit tests

### Fixed

- Add `absolute_mouse` value to `joystick_move` value, instead of override it (Server)

## [0.1.1] - 2024-06-27

### Added

- `env_logger`, debug and trace log (Client)
- Save config to file [#2](https://github.com/DiscreteTom/stickdeck-rs/issues/2) (Server)
- Dark mode [#1](https://github.com/DiscreteTom/stickdeck-rs/issues/1) (Server)
- `env_logger`, trace log (Server)

### Changed

- Default input update interval to 3ms to reach the max update rate (250+ updates per second) (Server)

### Fixed

- Scaling in gaming mode [#3](https://github.com/DiscreteTom/stickdeck-rs/issues/3) (Server)

## [0.1.0] - 2024-06-22

### Added

- Initial release

[Unreleased]: https://github.com/DiscreteTom/stickdeck-rs/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/DiscreteTom/stickdeck-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/DiscreteTom/stickdeck-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/DiscreteTom/stickdeck-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DiscreteTom/stickdeck-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/DiscreteTom/stickdeck-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DiscreteTom/stickdeck-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DiscreteTom/stickdeck-rs/releases/tag/v0.1.0
