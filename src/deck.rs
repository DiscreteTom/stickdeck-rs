mod action_repo;
mod all_deck_ctrl;
mod input;
mod ui;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub fn main() -> iced::Result {
  let update_lock = Arc::new(Mutex::new(false));
  let (tx, rx) = mpsc::channel::<String>();

  input::spawn(
    480, // TODO: replace 480 with the real AppID
    10,  // interval of polling input events
    update_lock.clone(),
    tx,
  )
  .unwrap();

  ui::run(
    30, // interval of updating UI
    update_lock,
    rx,
  )
}
