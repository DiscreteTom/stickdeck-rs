mod server;

use local_ip_address::local_ip;
use std::{env, net::SocketAddr, sync::mpsc};
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

    xbox
      .update(&data)
      .expect("Failed to update the virtual controller");
  }
}
