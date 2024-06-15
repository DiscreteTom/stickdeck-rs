mod client;
mod gamepad;
mod input;
mod ui;
mod utils;

use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
  let update_lock = Arc::new(Mutex::new(false));
  let (ui_tx, ui_rx) = mpsc::channel();
  let (net_tx, net_rx) = mpsc::channel();

  // TODO: make addr & port configurable
  client::spawn(SocketAddr::from(([192, 168, 1, 7], 7777)), net_rx);

  input::spawn(
    480, // TODO: replace 480 with the real AppID
    10,  // interval of polling input events // TODO: make this configurable
    update_lock.clone(),
    ui_tx,
    net_tx,
  )
  .expect("Failed to spawn input thread");

  ui::run(
    30, // interval of updating UI // TODO: make this configurable
    update_lock,
    ui_rx,
  )
  .expect("Failed to run the UI");
}
