To support the StickDeck project on Linux, where the client side emulates the Steam Deck as a virtual game controller (similar to an Xbox 360 controller on Windows via ViGEm Bus Driver), the primary adaptation involves replacing the Windows-specific ViGEm implementation with Linux's native uinput kernel module. Uinput enables the creation of virtual input devices from user space, allowing emulation of gamepads, keyboards, or other input hardware without requiring a separate bus driver like ViGEm. This approach is standard for Linux and integrates directly with the kernel's input subsystem (evdev).

Below, I outline the key steps to implement Linux support in the StickDeck client, assuming the project remains Rust-based. These steps build on the existing architecture: the server on the Steam Deck captures and streams inputs, while the client on the Linux PC receives them and emulates a virtual controller.

### Key Considerations for Linux Support
- **Permissions**: Accessing `/dev/uinput` typically requires root privileges or membership in the `input` group (e.g., via `sudo usermod -aG input $USER`). For production use, consider running the client with elevated permissions or using a setuid wrapper.
- **Event Types**: To emulate a gamepad, the virtual device must support event types such as `EV_KEY` (for buttons), `EV_ABS` (for analog sticks, triggers, and gyro/trackpad mappings), and optionally `EV_REL` (for relative mouse-like movements if needed).
- **Compatibility**: The emulated device will appear as a standard evdev device, compatible with most games and tools (e.g., SDL, Godot, or Steam) that read from `/dev/input/event*`.
- **Limitations**: Unlike ViGEm, uinput does not emulate specific proprietary protocols (e.g., exact Xbox 360 HID reports). However, it can mimic generic gamepads effectively. Test with tools like `jstest` or `evtest` to verify.

### Steps to Implement Linux Client Support
1. **Add Platform-Specific Code**: Modify the StickDeck client to use conditional compilation (e.g., `#[cfg(target_os = "linux")]` in Rust) for Linux-specific logic. Retain the Windows ViGEm code under `#[cfg(target_os = "windows")]`.

2. **Select a Rust Crate for Uinput Integration**: Use a Rust library to interface with uinput. Based on available options, the following are suitable alternatives:
   - **input-linux**: A maintained crate providing both evdev (for reading inputs) and uinput (for writing/virtualizing). It is designed for creating virtual input devices on Linux.
     - Latest version: 0.7.1 (as of recent updates).
     - Dependencies: Minimal; relies on `libc` for system calls.
     - Features: Supports creating devices with custom names, enabling specific event types (e.g., buttons, axes), and writing events synchronously.
     - Documentation: Available at https://docs.rs/input-linux.
   - **uinput**: An older, simpler wrapper for uinput syscalls, suitable for basic virtual devices.
     - Latest version: 0.1.3.
     - Dependencies: Primarily `libc`.
     - Features: Focuses on device creation and event emission; less comprehensive than input-linux but lightweight.
     - Repository: https://github.com/meh/rust-uinput.

   Recommendation: Start with `input-linux` for its active maintenance and comprehensive API. Add it to your `Cargo.toml`:
   ```
   [dependencies]
   input-linux = "0.7"
   ```

3. **Create the Virtual Gamepad**:
   - Open the uinput device (e.g., `/dev/uinput`).
   - Define the virtual device's properties: Set a name (e.g., "StickDeck Virtual Controller"), enable relevant event types, and configure axes (e.g., for left/right sticks with ranges -32768 to 32767) and buttons (e.g., A/B/X/Y, triggers).
   - Register the device to make it visible in the system.

   Example using `input-linux` (adapted from documentation; ensure error handling in production):
   ```rust
   use input_linux::{UinputHandle, EventKind, Key, AbsoluteAxis, UinputSetup};
   use std::fs::File;
   use std::io;

   fn create_virtual_gamepad() -> io::Result<UinputHandle<File>> {
       let file = File::create("/dev/uinput")?;
       let mut handle = UinputHandle::new(file);

       // Setup device properties
       let setup = UinputSetup::new("StickDeck Virtual Controller", 0x1234, 0x5678, 0x01); // Vendor, product, version
       handle.setup(&setup)?;

       // Enable button events (e.g., gamepad buttons)
       handle.enable(EventKind::Key)?;
       handle.enable_key(Key::BtnA)?;
       handle.enable_key(Key::BtnB)?;
       // Add more buttons as needed...

       // Enable absolute axes (e.g., for joysticks and triggers)
       handle.enable(EventKind::Absolute)?;
       handle.enable_absolute(AbsoluteAxis::X, -32768..=32767, 0, 0, 0, 0)?;
       handle.enable_absolute(AbsoluteAxis::Y, -32768..=32767, 0, 0, 0, 0)?;
       // Add RX/RY for right stick, Hat0X/Hat0Y for D-pad, etc.

       // Create the device
       handle.create()?;

       Ok(handle)
   }
   ```

4. **Handle Incoming Inputs**:
   - In the client's main loop, receive streamed data from the Steam Deck server (as in the current implementation).
   - Map received inputs (e.g., button presses, axis movements, gyro data) to uinput events.
   - Write events to the virtual device handle. For example:
     ```rust
     use input_linux::{AbsoluteEvent, KeyEvent, InputEvent};

     // Assuming 'handle' is the UinputHandle from above
     // For a button press:
     let btn_event = KeyEvent::new(0, Key::BtnA.into(), 1); // 1 for press, 0 for release
     handle.write(&[InputEvent::from(btn_event).into()])?;

     // For axis movement (e.g., left stick X):
     let axis_event = AbsoluteEvent::new(0, AbsoluteAxis::X.into(), 10000); // Value in range
     handle.write(&[InputEvent::from(axis_event).into()])?;

     // Synchronize events
     handle.synchronize()?;
     ```
   - For trackpad/gyro support: Map to additional axes (e.g., custom ABS_MISC) or relative events if simulating mouse input.

5. **Testing and Integration**:
   - Run the client with the server active, ensuring the PC and Steam Deck are networked.
   - Verify the virtual device appears (e.g., via `ls /dev/input/` or `evtest`).
   - Test inputs using Linux tools like `jstest /dev/input/js0` or in games.
   - Handle cleanup: Destroy the virtual device on exit to avoid lingering entries.

6. **Potential Enhancements**:
   - Add configurable options for axis sensitivity, dead zones, or mouse emulation (e.g., mapping trackpad to `EV_REL` for relative movements).
   - For cross-platform consistency, abstract the emulation logic behind a trait (e.g., `trait ControllerEmulator` with implementations for ViGEm and uinput).
   - If advanced features are needed (e.g., force feedback), explore extensions, though uinput primarily focuses on input emission.

This implementation should achieve functional parity with the Windows version. If the project requires more complex HID emulation, consider combining uinput with custom userspace drivers, but for standard gamepad use, uinput suffices. For further details, consult the crate documentation or Linux kernel uinput references.