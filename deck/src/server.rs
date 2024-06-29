use crate::gamepad::XGamepad;
use log::info;
use std::{
  io::{self, Write},
  net::{TcpListener, TcpStream},
  sync::mpsc,
  thread,
};
use stickdeck_common::{Packet, PACKET_FRAME_SIZE};

pub fn spawn(addr: &str, connected_tx: mpsc::Sender<mpsc::Sender<Packet<XGamepad>>>) {
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

    let mut buf = [0; PACKET_FRAME_SIZE];

    while let Ok(data) = data_rx.recv() {
      data.serialize(&mut buf);
      if write_stream(&mut stream, &buf).is_err() {
        break;
      }
    }

    info!("Client disconnected");
  });
}

fn write_stream(stream: &mut TcpStream, buf: &[u8; PACKET_FRAME_SIZE]) -> io::Result<()> {
  stream.write_all(buf)?;
  stream.flush()?;
  Ok(())
}

include!("../../snippet/serialize.rs");

#[cfg(test)]
mod tests {
  use super::*;
  use crate::gamepad::XButtons;
  use stickdeck_common::MouseMove;

  include!("../../snippet/deserialize.rs");
  include!("../../snippet/test_serialize_deserialize.rs");
}
