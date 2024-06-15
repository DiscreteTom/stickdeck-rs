mod client;

use std::sync::mpsc;
use vigem_client::{Client, TargetId, Xbox360Wired};

fn main() {
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

  let (gamepad_tx, gamepad_rx) = mpsc::channel();

  // TODO: customizable server address
  println!("Connecting to the server...");
  client::spawn(&format!("{}:{}", "steamdeck", 7777), gamepad_tx);

  while let Ok(data) = gamepad_rx.recv() {
    // println!("{:?}", data);

    xbox
      .update(&data)
      .expect("Failed to update the virtual controller");
  }

  println!("Shutting down...");
}
