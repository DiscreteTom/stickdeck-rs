mod client;

use log::{debug, info, log_enabled, trace, Level};
use std::{env, sync::mpsc, time::Instant};
use stickdeck_common::{MouseMove, Packet};
use vigem_client::{Client, TargetId, XGamepad, Xbox360Wired};
use windows::Win32::UI::Input::KeyboardAndMouse::{
  SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEINPUT, MOUSE_EVENT_FLAGS,
};

fn main() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info")
  }
  env_logger::init();

  info!("stickdeck-win v{}", env!("CARGO_PKG_VERSION"));
  info!("See https://github.com/DiscreteTom/stickdeck-rs for more info.");

  let (gamepad_tx, gamepad_rx) = mpsc::channel();

  // connect to the server
  client::spawn(
    &format!(
      "{}:{}",
      env::args().nth(1).unwrap_or("steamdeck".to_string()),
      7777
    ),
    gamepad_tx,
  );

  let mut update_controller = init_controller();
  let mut move_mouse = init_mouse();

  let mut now = Instant::now();
  let mut count = 0;
  while let Ok(data) = gamepad_rx.recv() {
    trace!("Got {:?}", data);

    match data {
      Packet::Timestamp(_timestamp) => {} // TODO
      Packet::Gamepad(gamepad) => update_controller(&gamepad),
      Packet::MouseMove(data) => move_mouse(&data),
      Packet::GamepadAndMouseMove(gamepad, data) => {
        update_controller(&gamepad);
        move_mouse(&data);
      }
    }

    if log_enabled!(Level::Debug) {
      count += 1;
      if now.elapsed().as_secs() >= 1 {
        debug!("{} updates per second", count);
        now = Instant::now();
        count = 0;
      }
    }
  }

  info!("Shutting down...");
}

fn init_controller() -> impl FnMut(&XGamepad) {
  let mut xbox = Xbox360Wired::new(
    Client::connect().expect("Failed to connect to the ViGEmBus driver"),
    TargetId::XBOX360_WIRED,
  );
  xbox
    .plugin()
    .expect("Failed to plugin the virtual controller");
  xbox
    .wait_ready()
    .expect("Failed to wait for the virtual controller to be ready");

  info!("Virtual controller is ready");

  move |data| {
    xbox
      .update(data)
      .expect("Failed to update the virtual controller")
  }
}

fn init_mouse() -> impl FnMut(&MouseMove) {
  move |data: &MouseMove| unsafe {
    let input = INPUT {
      r#type: INPUT_MOUSE,
      Anonymous: INPUT_0 {
        mi: MOUSEINPUT {
          dx: data.x as i32,
          dy: data.y as i32,
          mouseData: 0,
          dwFlags: MOUSE_EVENT_FLAGS(0x0001),
          time: 0,
          dwExtraInfo: 0,
        },
      },
    };
    SendInput(&[input], std::mem::size_of_val(&input) as i32);
  }
}
