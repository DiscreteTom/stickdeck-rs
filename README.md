# Stickdeck

[![version](https://img.shields.io/github/v/tag/DiscreteTom/stickdeck-rs?label=release&style=flat-square)](https://github.com/DiscreteTom/stickdeck-rs/releases/latest)
![license](https://img.shields.io/github/license/DiscreteTom/stickdeck-rs?style=flat-square)
![rust](https://img.shields.io/badge/built_with-rust-DEA584?style=flat-square)

Turn your SteamDeck into a joystick for your PC, with trackpad and gyro support!

## Setup

### Server Side (SteamDeck)

#### Install from Steam

TODO

#### Install from GitHub

1. **_Switch to Desktop Mode on SteamDeck. All the following steps are done on SteamDeck in the desktop mode._**
2. Download `stickdeck-x.x.x.zip` from the [latest release](https://github.com/DiscreteTom/stickdeck-rs/releases/latest) and extract it.
3. Run `setup.sh` in the extracted folder.
4. Run `stickdeck` on SteamDeck, this should open a new window, but your input is not captured now. Close the window.
5. Start `Steam` client (NOT `Returning to Gaming Mode`) in the Desktop Mode. In your library, you should find a game called `Spacewar`. [Edit its input mapping](https://partner.steamgames.com/doc/features/steam_controller/getting_started_for_devs#14). You should have a layout called `stickdeck`, edit it so that all the inputs are mapped to the correct game actions.
   1. For joysticks, use `Left/Right Stick (as Joystick Move)`.
   2. For trackpad and gyro, use `Left/Right Stick (as Absolute Mouse)`.
6. Run `stickdeck` again, click `Start Server`, now you should see the input when you press buttons or move joysticks on SteamDeck.

Now the setup is done, proceed to the client side.

> Next time you want to use Stickdeck, you only need to run `stickdeck` and click `Start Server`.

### Client Side (PC)

1. Install [ViGEm Bus Driver](https://github.com/nefarius/ViGEmBus) and **_restart_** your PC.
2. Download `stickdeck-win-x.x.x.zip` from the [latest release](https://github.com/DiscreteTom/stickdeck-rs/releases/latest) and extract it.
3. Make sure your SteamDeck and your PC are in the same network.
4. Make sure your SteamDeck is running the server.
5. Run `stickdeck-win.exe`. Once your see `Virtual controller is ready` in the console, Stickdeck is ready.
6. (Optional) If you want to test the controller, run `joy.cpl`.

## Credit

- [ViGEm Bus Driver](https://github.com/nefarius/ViGEmBus)
- [stickdeck (PoC version made with Python)](https://github.com/DiscreteTom/stickdeck)
- [kontroller](https://github.com/DiscreteTom/kontroller/)

## [CHANGELOG](./CHANGELOG.md)
