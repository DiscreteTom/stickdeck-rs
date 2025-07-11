use anyhow::{Context, Result};
use input_linux::{
	EventKind, EventTime, InputEvent, Key, KeyEvent, KeyState, RelativeAxis, RelativeEvent,
	SynchronizeEvent, UInputHandle,
};
use log::debug;
use std::fs::OpenOptions;
use stickdeck_common::{Mouse, MouseButton};

pub fn init_mouse() -> Result<impl FnMut(&Mouse) -> Result<()>> {
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.open("/dev/uinput")
		.context("Failed to open /dev/uinput. Make sure you have permissions (try running with sudo or add user to 'input' group)")?;

	let handle = UInputHandle::new(file);

	// Enable key events for mouse buttons
	handle
		.set_evbit(EventKind::Key)
		.context("Failed to enable key events")?;
	handle
		.set_evbit(EventKind::Relative)
		.context("Failed to enable relative events")?;

	// Enable mouse buttons
	handle.set_keybit(Key::ButtonLeft)?;
	handle.set_keybit(Key::ButtonRight)?;
	handle.set_keybit(Key::ButtonMiddle)?;

	// Enable relative axes for movement and scroll
	handle.set_relbit(RelativeAxis::X)?;
	handle.set_relbit(RelativeAxis::Y)?;
	handle.set_relbit(RelativeAxis::Wheel)?;

	// Create the virtual mouse device
	handle
		.create(
			&input_linux::InputId {
				bustype: input_linux::sys::BUS_USB,
				vendor: 0x0000,
				product: 0x0000,
				version: 0x0001,
			},
			b"StickDeck Virtual Mouse",
			0,
			&[],
		)
		.context("Failed to create virtual mouse")?;

	debug!("Virtual mouse created successfully");

	let mut prev_buttons = MouseButton::empty();

	Ok(move |mouse: &Mouse| -> Result<()> {
		let time = EventTime::new(0, 0);
		let mut events = Vec::new();

		// Handle mouse movement
		if mouse.x != 0 {
			events.push(InputEvent::from(RelativeEvent::new(
				time,
				RelativeAxis::X,
				mouse.x as i32,
			)));
		}

		if mouse.y != 0 {
			events.push(InputEvent::from(RelativeEvent::new(
				time,
				RelativeAxis::Y,
				mouse.y as i32,
			)));
		}

		// Handle scroll wheel
		if mouse.scroll != 0 {
			events.push(InputEvent::from(RelativeEvent::new(
				time,
				RelativeAxis::Wheel,
				-(mouse.scroll as i32), // Invert scroll direction for Linux
			)));
		}

		// Handle button state changes
		let button_mappings = [
			(MouseButton::LEFT, Key::ButtonLeft),
			(MouseButton::RIGHT, Key::ButtonRight),
			(MouseButton::MIDDLE, Key::ButtonMiddle),
		];

		for (mouse_button, linux_key) in &button_mappings {
			let current = mouse.buttons.contains(*mouse_button);
			let previous = prev_buttons.contains(*mouse_button);
			if current != previous {
				events.push(InputEvent::from(KeyEvent::new(
					time,
					*linux_key,
					KeyState::pressed(current),
				)));
			}
		}

		// Write events if any
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
				.context("Failed to write mouse events")?;
			debug!("Updated mouse state with {} events", event_count);
		}

		prev_buttons = mouse.buttons;
		Ok(())
	})
}