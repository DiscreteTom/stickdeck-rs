mod client;
mod gamepad;
mod input;
mod ui;
mod utils;

use std::net::SocketAddr;
use std::sync::mpsc;

fn main() {
  let (update_tx, update_rx) = mpsc::channel();
  let (ui_tx, ui_rx) = mpsc::channel();
  let (net_tx, net_rx) = mpsc::channel();

  // TODO: make addr & port configurable
  client::spawn(SocketAddr::from(([192, 168, 1, 7], 7777)), net_rx);

  input::spawn(
    480, // TODO: replace 480 with the real AppID
    10,  // interval of polling input events // TODO: make this configurable
    update_rx, ui_tx, net_tx,
  )
  .expect("Failed to spawn input thread");

  ui::run(
    30, // interval of updating UI // TODO: make this configurable
    update_tx, ui_rx,
  )
  .expect("Failed to run the UI");
}
