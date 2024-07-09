# CHANGELOG

## Next

- Client (PC)
  - Feat: add `launch.bat` to start the client side and create a log file for debugging.
  - Feat: add retry when connecting to the server.
  - Fix: `debug.bat` will pause on error.
- Server (Steam Deck)
  - Feat: create a log file for debugging.

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
