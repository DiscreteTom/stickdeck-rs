# CHANGELOG

## v0.3.2

- Server (Steam Deck)
  - Fix: fix memory leak, remove debug info toggle.

## v0.3.1

- Client (PC)
  - Feat: add performance warning in debug log level.
- Server (Steam Deck)
  - Feat: add performance warning in debug log level.
  - Feat: less unnecessary logs in debug mode.
  - Fix: add a toggle to disable debug info to prevent memory leak.

## v0.3.0

- **_Breaking Change_**: update network protocol and action set.
- Client (PC)
  - Feat: add `launch.bat` to start the client side and create a log file for debugging.
  - Feat: add retry when connecting to the server.
  - Feat: support left & right mouse buttons and scroll wheel.
  - Fix: `debug.bat` will pause on error.
- Server (Steam Deck)
  - **_Breaking Change_**: remove `LeftMouse` and `RightMouse` from the action set, use `MouseMove` instead.
  - Feat: create a log file for debugging.
  - Feat: add `Left Mouse Button`, `Right Mouse Button` and `Mouse Scroll` to the action set.

## v0.2.1

- Client (PC)
  - Fix: apply [`SendInput`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput) API instead of [`SetCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos) to move the mouse cursor for a better compatibility against games.

## v0.2.0

- **_Breaking Change_**: update network protocol.
- Client (PC)
  - Feat: `absolute_mouse` actions will control the mouse cursor directly.
- Server (Steam Deck)
  - Note: increase vertical padding to fit performance overlay level 2.
  - Note: `debug.sh` now will set `RUST_LOG=debug`.

## v0.1.2

- Client (PC)
  - Note: auto build the client side with GitHub Actions.
- Server (Steam Deck)
  - Fix: add `absolute_mouse` value to `joystick_move` value, instead of override it.
- Note: add unit tests.

## v0.1.1

- Client (PC)
  - Note: apply `env_logger`, add debug and trace log.
- Server (Steam Deck)
  - Feat: save config to file. [#2](https://github.com/DiscreteTom/stickdeck-rs/issues/2)
  - Feat: add dark mode. [#1](https://github.com/DiscreteTom/stickdeck-rs/issues/1)
  - Note: change default input update interval to 3ms to reach the max update rate (250+ updates per second).
  - Note: apply `env_logger`, add trace log.
  - Fix: fix scaling in gaming mode. [#3](https://github.com/DiscreteTom/stickdeck-rs/issues/3)

## v0.1.0

The initial release.
