mod server;

use local_ip_address::local_ip;
use std::{env, net::SocketAddr, sync::mpsc};
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

  server::spawn(
    SocketAddr::new(
      local_ip().expect("Failed to get local ip address"),
      env::var("STICKDECK_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(7777),
    ),
    action_tx,
  );

  loop {
    let data = action_rx
      .recv()
      .expect("Failed to receive data from the server");

    // println!("{:?}", data);

    target
      .update(&data)
      .expect("Failed to update the virtual controller");
  }
}
