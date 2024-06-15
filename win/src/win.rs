mod client;

use std::{env, sync::mpsc};
use vigem_client::{Client, TargetId, Xbox360Wired};

fn main() {
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
  println!("Virtual controller is ready");

  while let Ok(data) = gamepad_rx.recv() {
    // println!("{:?}", data);

    xbox
      .update(&data)
      .expect("Failed to update the virtual controller");
  }

  println!("Shutting down...");
}
