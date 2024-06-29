mod client;

use log::{debug, info, log_enabled, trace, Level};
use mouse_rs::Mouse;
use std::{env, sync::mpsc, time::Instant};
use stickdeck_common::Packet;
use vigem_client::{Client, TargetId, Xbox360Wired};

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

  // setup the virtual controller
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

  // init mouse
  let mouse = Mouse::new();

  let mut now = Instant::now();
  let mut count = 0;
  while let Ok(data) = gamepad_rx.recv() {
    trace!("Got {:?}", data);

    match data {
      Packet::Timestamp(_timestamp) => todo!(), // TODO
      Packet::GamePad(gamepad) => {
        xbox
          .update(&gamepad)
          .expect("Failed to update the virtual controller");
      }
      Packet::MouseMove(data) => {
        if data.x != 0 || data.y != 0 {
          let pos = mouse
            .get_position()
            .expect("Failed to get the mouse position");
          mouse
            .move_to(pos.x + data.x as i32, pos.y + data.y as i32)
            .expect("Failed to move the mouse");
        }
      }
      Packet::GamePadAndMouseMove(gamepad, data) => {
        // TODO: optimize code
        xbox
          .update(&gamepad)
          .expect("Failed to update the virtual controller");
        if data.x != 0 || data.y != 0 {
          let pos = mouse
            .get_position()
            .expect("Failed to get the mouse position");
          mouse
            .move_to(pos.x + data.x as i32, pos.y + data.y as i32)
            .expect("Failed to move the mouse");
        }
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
