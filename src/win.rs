mod serde;
mod server;

use std::sync::mpsc;
use vigem_client::{Client, TargetId, Xbox360Wired};

fn main() {
  let client = Client::connect().expect("Failed to connect to the ViGEmBus driver");
  let id = TargetId::XBOX360_WIRED;
  let mut target = Xbox360Wired::new(client, id);
  target
    .plugin()
    .expect("Failed to plugin the virtual controller");
  target
    .wait_ready()
    .expect("Failed to wait for the virtual controller to be ready");
  println!("Virtual controller is ready");

  let (action_tx, action_rx) = mpsc::channel();

  server::spawn(7777, action_tx);

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
