# StickDeck

[![version](https://img.shields.io/github/v/tag/DiscreteTom/stickdeck-rs?label=release&style=flat-square)](https://github.com/DiscreteTom/stickdeck-rs/releases/latest)
![license](https://img.shields.io/github/license/DiscreteTom/stickdeck-rs?style=flat-square)
![rust](https://img.shields.io/badge/built_with-rust-DEA584?style=flat-square)

Turn your Steam Deck into a joystick for your PC, with trackpad and gyro support!

## Setup

### Server Side (Steam Deck)

> [!NOTE]
> The Steam version and the GitHub version are totally the same,
> but setup via Steam is way more easier for most users,
> and you can also get automatic updates.

> [!IMPORTANT]
> The server side will be treated as a game on Steam Deck, so you can't play Steam games on PC while the server side is running on Steam Deck <ins>**_with the same Steam account_**</ins>. You can still play non-Steam games on PC, or you can register a new Steam account for the server side.

#### Install from Steam

Just acquire the StickDeck from Steam and download it to your Steam Deck, then no further setup is needed.

Now you can proceed to the [client side setup](#client-side-pc).

#### Install from GitHub

1. **_Switch to Desktop Mode on Steam Deck. All the following steps are done on Steam Deck in the desktop mode._**
2. Download `stickdeck-vX.X.X.zip` from the [latest release](https://github.com/DiscreteTom/stickdeck-rs/releases/latest) and extract it.
3. Run `setup.sh` in the extracted folder.
4. Run `launch.sh` on Steam Deck, this should open a new window, but your input is not captured now. Close the window by tapping the `Exit` button.
5. Start `Steam` client (NOT `Returning to Gaming Mode`) in the Desktop Mode. In your library, you should find a game called `Spacewar`. [Edit its input mapping](https://partner.steamgames.com/doc/features/steam_controller/getting_started_for_devs#14) so that all the inputs are mapped to the correct game actions.
   1. For joysticks, use `Left/Right Stick (as Joystick Move)`.
   2. For trackpad and gyro, use `Left/Right Stick (as Absolute Mouse)`.
6. Run `launch.sh` again, click `Start Server`, now you should see the input when you press buttons or move joysticks on Steam Deck.
7. Now you can exit by tapping the `Exit` button. Next time you want to start the server, just run `launch.sh` and click `Start Server`.

> You can also add `launch.sh` as a non-Steam game on Steam Deck, so you can start the server directly from Steam Deck's Gaming Mode.

### Client Side (PC)

1. Install [ViGEm Bus Driver](https://github.com/nefarius/ViGEmBus) and **_restart_** your PC.
2. Download `stickdeck-win-vX.X.X.zip` from the [latest release](https://github.com/DiscreteTom/stickdeck-rs/releases/latest) and extract it.

## Usage

1. Start the server on Steam Deck. Make sure the server is running and the input is captured.
2. Make sure your PC and your Steam Deck are in the same network.
3. Make sure the client on your PC is under the same minor version as the server on Steam Deck.
4. Run `launch.bat` on your PC. Once you see `Virtual controller is ready` in the console, StickDeck is ready.
5. (Optional) If you want to test the controller, run `joy.cpl`.

> [!NOTE]
> By default the client will try to connect `steamdeck:7777`. If you want to connect to a different server, you can edit `launch.bat`, replace the `steamdeck` with your server IP.
> You can find the server IP on the first line of the StickDeck UI window while the server is started.

## FAQ

- Poll/update rate?
  - Depends on the configurable input update interval. In my case, set the input update interval to 3ms to reach the max update rate of 250+Hz.
  - Besides, the server side will only send the input when there is a change, so the actual update rate will be lower than the configured rate.
  - You can checkout the actual update rate on the PC side by running `debug.bat`.

## Credit

- [ViGEm Bus Driver](https://github.com/nefarius/ViGEmBus)
- [stickdeck (PoC version made with Python)](https://github.com/DiscreteTom/stickdeck)
- [kontroller](https://github.com/DiscreteTom/kontroller/)

## [CHANGELOG](./CHANGELOG.md)
