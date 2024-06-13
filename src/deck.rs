mod client;
mod input;
mod input_action;
mod serde;
mod ui;
mod utils;
mod xbox_ctrl;

use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
  let update_lock = Arc::new(Mutex::new(false));
  let (tx, rx) = mpsc::channel::<String>();

  let (net_tx, net_rx) = mpsc::channel();

  thread::spawn(move || client::start(SocketAddr::from(([192, 168, 1, 7], 7777)), net_rx));

  input::spawn(
    480, // TODO: replace 480 with the real AppID
    10,  // interval of polling input events
    update_lock.clone(),
    tx,
    net_tx,
  )
  .unwrap();

  ui::run(
    30, // interval of updating UI
    update_lock,
    rx,
  )
  .expect("Failed to run the UI");
}
