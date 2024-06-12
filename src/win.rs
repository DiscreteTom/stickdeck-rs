mod model;
mod server;

use server::TheStickdeckServer;
use tokio::sync::mpsc;
use vigem_client::{Client, TargetId, XGamepad, Xbox360Wired};

#[tokio::main]
async fn main() {
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

  let mut gamepad = XGamepad::default();

  let (action_tx, mut action_rx) = mpsc::channel(10);
  TheStickdeckServer::start(7777, action_tx).await;

  loop {
    let data = action_rx
      .recv()
      .await
      .expect("Failed to receive data from the server");

    match data {
      server::Action::UpdateGamepad(data) => {
        gamepad.buttons.raw = (data.button_trigger >> 16) as u16;
        gamepad.left_trigger = (data.button_trigger >> 8) as u8;
        gamepad.right_trigger = data.button_trigger as u8;
        gamepad.thumb_lx = (data.thumb >> 48) as i16;
        gamepad.thumb_ly = (data.thumb >> 32) as i16;
        gamepad.thumb_rx = (data.thumb >> 16) as i16;
        gamepad.thumb_ry = data.thumb as i16;
      }
    }

    target
      .update(&gamepad)
      .expect("Failed to update the virtual controller");
  }
}
