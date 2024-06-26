# StickDeck

[![version](https://img.shields.io/github/v/tag/DiscreteTom/stickdeck-rs?label=release&style=flat-square)](https://github.com/DiscreteTom/stickdeck-rs/releases/latest)
![license](https://img.shields.io/github/license/DiscreteTom/stickdeck-rs?style=flat-square)
![rust](https://img.shields.io/badge/built_with-rust-DEA584?style=flat-square)

Turn your SteamDeck into a joystick for your PC, with trackpad and gyro support!

## Setup

### Server Side (SteamDeck)

> [!NOTE]
> The Steam version and the GitHub version are totally the same,
> but configuring via Steam is way more easier for most users.

> [!IMPORTANT]
> The server side will be treated as a game on SteamDeck, so you can't play Steam games on PC while the server side is running on SteamDeck **_with the same Steam account_**. You can still play non-Steam games on PC, or you can register a new Steam account for the server side.

#### Install from Steam

TODO

#### Install from GitHub

1. **_Switch to Desktop Mode on SteamDeck. All the following steps are done on SteamDeck in the desktop mode._**
2. Download `stickdeck-vX.X.X.zip` from the [latest release](https://github.com/DiscreteTom/stickdeck-rs/releases/latest) and extract it.
3. Run `setup.sh` in the extracted folder.
4. Run `launch.sh` on SteamDeck, this should open a new window, but your input is not captured now. Close the window by tapping the `Exit` button.
5. Start `Steam` client (NOT `Returning to Gaming Mode`) in the Desktop Mode. In your library, you should find a game called `Spacewar`. [Edit its input mapping](https://partner.steamgames.com/doc/features/steam_controller/getting_started_for_devs#14) so that all the inputs are mapped to the correct game actions.
   1. For joysticks, use `Left/Right Stick (as Joystick Move)`.
   2. For trackpad and gyro, use `Left/Right Stick (as Absolute Mouse)`.
6. Run `launch.sh` again, click `Start Server`, now you should see the input when you press buttons or move joysticks on SteamDeck.

Now the setup is done, you can proceed to the client side.

Next time you want to use StickDeck, you only need to run `launch.sh` and click `Start Server`.

### Client Side (PC)

1. Install [ViGEm Bus Driver](https://github.com/nefarius/ViGEmBus) and **_restart_** your PC.
2. Download `stickdeck-win-vX.X.X.zip` from the [latest release](https://github.com/DiscreteTom/stickdeck-rs/releases/latest) and extract it.
3. Make sure your SteamDeck and your PC are in the same network.
4. Make sure your SteamDeck is running the server.
5. Run `stickdeck-win.exe`. Once your see `Virtual controller is ready` in the console, StickDeck is ready.
6. (Optional) If you want to test the controller, run `joy.cpl`.

Next time you want to use StickDeck, you only need to run `stickdeck-win.exe` (of course you should start the server first).

> [!NOTE]
> By default the client will try to connect `steamdeck:7777`. If you want to connect to a different server, you can run `stickdeck-win.exe [server-ip]`.
> You can find the server IP on the first line of the StickDeck UI window while the server is started.

## FAQ

- Poll/update rate?
  - Depends on the configurable input update interval. In my case, set the input update interval to 3ms to reach the max update rate of 251Hz.
  - Besides, the server side will only send the input when there is a change, so the actual update rate will be lower than the configured rate.

## Credit

- [ViGEm Bus Driver](https://github.com/nefarius/ViGEmBus)
- [stickdeck (PoC version made with Python)](https://github.com/DiscreteTom/stickdeck)
- [kontroller](https://github.com/DiscreteTom/kontroller/)

## [CHANGELOG](./CHANGELOG.md)
