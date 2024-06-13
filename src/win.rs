mod serde;
mod server;

use std::{sync::mpsc, thread};
use vigem_client::{Client, TargetId, Xbox360Wired};

fn main() {
  let client = Client::connect().expect("Failed to connect to the ViGEmBus driver");
  let id = TargetId::XBOX360_WIRED;
  let mut target = Xbox360Wired::new(client, id);

  // Plugin the virtual controller
  target
    .plugin()
    .expect("Failed to plugin the virtual controller");

  // Wait for the virtual controller to be ready to accept updates
  target
    .wait_ready()
    .expect("Failed to wait for the virtual controller to be ready");

  let (action_tx, action_rx) = mpsc::channel();

  thread::spawn(move || server::start(7777, action_tx));

  loop {
    let data = action_rx
      .recv()
      .expect("Failed to receive data from the server");

    println!("Received data: {:?}", data);

    target
      .update(&data)
      .expect("Failed to update the virtual controller");
  }
}
