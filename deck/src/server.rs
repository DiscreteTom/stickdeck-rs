use crate::gamepad::XGamepad;
use log::info;
use std::{io::Write, net::TcpListener, sync::mpsc, thread};

pub fn spawn(addr: &str, connected_tx: mpsc::Sender<mpsc::Sender<XGamepad>>) {
  let listener = TcpListener::bind(addr).expect(&format!("Failed to bind to address {}", addr));

  info!("Server listening on {}", addr);

  thread::spawn(move || {
    // only accept one client because we will consume the receiver
    let mut stream = listener
      .incoming()
      .next()
      .unwrap()
      .expect("Failed to accept connection");
    stream.set_nodelay(true).expect("Failed to set nodelay");
    info!("New client connected");

    let (data_tx, data_rx) = mpsc::channel();

    connected_tx
      .send(data_tx)
      .expect("Failed to send connected signal");

    while let Ok(data) = data_rx.recv() {
      if stream
        .write_all(&serialize(&data))
        .and_then(|_| stream.flush())
        .is_err()
      {
        break;
      }
    }

    info!("Client disconnected");
  });
}

include!("serialize.rs");

#[cfg(test)]
mod tests {
  use super::*;
  use crate::gamepad::XButtons;

  include!("../../win/src/deserialize.rs");
  include!("../../snippet/test_serialize_deserialize.rs");
}
