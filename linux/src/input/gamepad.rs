use anyhow::{Context, Result};
use input_linux::{
	AbsoluteAxis, AbsoluteEvent, AbsoluteInfo, AbsoluteInfoSetup, EventKind, EventTime,
	InputEvent, Key, KeyEvent, KeyState, SynchronizeEvent, UInputHandle,
};
use log::{debug, info};
use std::fs::OpenOptions;
use stickdeck_common::{XButtons, XGamepad};

const STICK_MAX: i32 = 32767;
const STICK_MIN: i32 = -32768;
const TRIGGER_MAX: i32 = 255;
const TRIGGER_MIN: i32 = 0;

pub fn init_gamepad() -> Result<impl FnMut(&XGamepad) -> Result<()>> {
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.open("/dev/uinput")
		.context("Failed to open /dev/uinput. Make sure you have permissions (try running with sudo or add user to 'input' group)")?;

	let handle = UInputHandle::new(file);

	handle
		.set_evbit(EventKind::Key)
		.context("Failed to enable key events")?;
	handle
		.set_evbit(EventKind::Absolute)
		.context("Failed to enable absolute events")?;

	// Enable gamepad buttons
	handle.set_keybit(Key::ButtonSouth)?; // A
	handle.set_keybit(Key::ButtonEast)?; // B
	handle.set_keybit(Key::ButtonNorth)?; // X
	handle.set_keybit(Key::ButtonWest)?; // Y
	handle.set_keybit(Key::ButtonTL)?; // Left Bumper
	handle.set_keybit(Key::ButtonTR)?; // Right Bumper
	handle.set_keybit(Key::ButtonSelect)?; // Back/View
	handle.set_keybit(Key::ButtonStart)?;
	handle.set_keybit(Key::ButtonMode)?; // Guide/Xbox button
	handle.set_keybit(Key::ButtonThumbl)?; // Left Stick Click
	handle.set_keybit(Key::ButtonThumbr)?; // Right Stick Click

	// D-Pad as hat axes
	let dpad_info = AbsoluteInfo {
		value: 0,
		minimum: -1,
		maximum: 1,
		fuzz: 0,
		flat: 0,
		resolution: 0,
	};

	// Setup analog sticks
	let stick_info = AbsoluteInfo {
		value: 0,
		minimum: STICK_MIN,
		maximum: STICK_MAX,
		fuzz: 16,
		flat: 128,
		resolution: 0,
	};

	// Setup triggers
	let trigger_info = AbsoluteInfo {
		value: 0,
		minimum: TRIGGER_MIN,
		maximum: TRIGGER_MAX,
		fuzz: 0,
		flat: 0,
		resolution: 0,
	};

	// Left stick
	handle.set_absbit(AbsoluteAxis::X)?;
	handle.set_absbit(AbsoluteAxis::Y)?;

	// Right stick
	handle.set_absbit(AbsoluteAxis::RX)?;
	handle.set_absbit(AbsoluteAxis::RY)?;

	// Triggers
	handle.set_absbit(AbsoluteAxis::Z)?; // Left trigger
	handle.set_absbit(AbsoluteAxis::RZ)?; // Right trigger

	// D-Pad
	handle.set_absbit(AbsoluteAxis::Hat0X)?;
	handle.set_absbit(AbsoluteAxis::Hat0Y)?;

	// Set absolute axis info
	let abs_info = vec![
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::X,
			info: stick_info.clone(),
		},
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::Y,
			info: stick_info.clone(),
		},
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::RX,
			info: stick_info.clone(),
		},
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::RY,
			info: stick_info.clone(),
		},
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::Z,
			info: trigger_info.clone(),
		},
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::RZ,
			info: trigger_info.clone(),
		},
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::Hat0X,
			info: dpad_info.clone(),
		},
		AbsoluteInfoSetup {
			axis: AbsoluteAxis::Hat0Y,
			info: dpad_info.clone(),
		},
	];

	// Create the virtual device
	handle
		.create(
			&input_linux::InputId {
				bustype: input_linux::sys::BUS_USB,
				vendor: 0x045e,  // Microsoft
				product: 0x028e, // Xbox 360 Controller
				version: 0x0110,
			},
			b"StickDeck Virtual Controller",
			0,
			&abs_info,
		)
		.context("Failed to create virtual gamepad")?;

	info!("Virtual gamepad created successfully");

	let mut prev_state = XGamepad::default();

	Ok(move |gamepad: &XGamepad| -> Result<()> {
		let time = EventTime::new(0, 0);
		let mut events = Vec::new();

		// Map buttons
		let buttons = [
			(XButtons::A, Key::ButtonSouth),
			(XButtons::B, Key::ButtonEast),
			(XButtons::X, Key::ButtonNorth),
			(XButtons::Y, Key::ButtonWest),
			(XButtons::LEFT_SHOULDER, Key::ButtonTL),
			(XButtons::RIGHT_SHOULDER, Key::ButtonTR),
			(XButtons::BACK, Key::ButtonSelect),
			(XButtons::START, Key::ButtonStart),
			(XButtons::GUIDE, Key::ButtonMode),
			(XButtons::LEFT_THUMB, Key::ButtonThumbl),
			(XButtons::RIGHT_THUMB, Key::ButtonThumbr),
		];

		for (xbox_button, linux_key) in &buttons {
			let current = gamepad.buttons.contains(*xbox_button);
			let previous = prev_state.buttons.contains(*xbox_button);
			if current != previous {
				events.push(InputEvent::from(KeyEvent::new(
					time,
					*linux_key,
					KeyState::pressed(current),
				)));
			}
		}

		// Map D-Pad
		let dpad_x = if gamepad.buttons.contains(XButtons::DPAD_RIGHT) {
			1
		} else if gamepad.buttons.contains(XButtons::DPAD_LEFT) {
			-1
		} else {
			0
		};

		let dpad_y = if gamepad.buttons.contains(XButtons::DPAD_DOWN) {
			1
		} else if gamepad.buttons.contains(XButtons::DPAD_UP) {
			-1
		} else {
			0
		};

		let prev_dpad_x = if prev_state.buttons.contains(XButtons::DPAD_RIGHT) {
			1
		} else if prev_state.buttons.contains(XButtons::DPAD_LEFT) {
			-1
		} else {
			0
		};

		let prev_dpad_y = if prev_state.buttons.contains(XButtons::DPAD_DOWN) {
			1
		} else if prev_state.buttons.contains(XButtons::DPAD_UP) {
			-1
		} else {
			0
		};

		if dpad_x != prev_dpad_x {
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::Hat0X,
				dpad_x,
			)));
		}

		if dpad_y != prev_dpad_y {
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::Hat0Y,
				dpad_y,
			)));
		}

		// Map analog sticks
		if gamepad.thumb_lx != prev_state.thumb_lx {
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::X,
				gamepad.thumb_lx as i32,
			)));
		}

		if gamepad.thumb_ly != prev_state.thumb_ly {
			// Invert Y axis for Linux
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::Y,
				-(gamepad.thumb_ly as i32),
			)));
		}

		if gamepad.thumb_rx != prev_state.thumb_rx {
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::RX,
				gamepad.thumb_rx as i32,
			)));
		}

		if gamepad.thumb_ry != prev_state.thumb_ry {
			// Invert Y axis for Linux
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::RY,
				-(gamepad.thumb_ry as i32),
			)));
		}

		// Map triggers
		if gamepad.left_trigger != prev_state.left_trigger {
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::Z,
				gamepad.left_trigger as i32,
			)));
		}

		if gamepad.right_trigger != prev_state.right_trigger {
			events.push(InputEvent::from(AbsoluteEvent::new(
				time,
				AbsoluteAxis::RZ,
				gamepad.right_trigger as i32,
			)));
		}

		// Write events if any changes occurred
		if !events.is_empty() {
			events.push(InputEvent::from(SynchronizeEvent::report(time)));
			
			let event_count = events.len() - 1;
			
			// Convert InputEvent to raw input_event
			let raw_events: Vec<input_linux::sys::input_event> = events
				.into_iter()
				.map(|e| *e.as_raw())
				.collect();
			
			handle
				.write(&raw_events)
				.context("Failed to write gamepad events")?;
			debug!("Updated gamepad state with {} events", event_count);
		}

		prev_state = gamepad.clone();
		Ok(())
	})
}