use crate::gamepad::{XButtons, XGamepad};
use log::info;
use std::{
  io::{self, Write},
  net::{TcpListener, TcpStream},
  sync::mpsc,
  thread,
};
use stickdeck_common::{perf, Packet, PACKET_FRAME_SIZE};

// Use macro to implement SerializableGamepad trait for production code
stickdeck_common::impl_serializable_gamepad!(XGamepad);

pub fn spawn(addr: &str, connected_tx: mpsc::Sender<mpsc::SyncSender<Packet<XGamepad>>>) {
  let listener =
    TcpListener::bind(addr).unwrap_or_else(|_| panic!("Failed to bind to address {}", addr));

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

    // use a bounded channel to prevent network buffer from growing too large
    let (data_tx, data_rx) = mpsc::sync_channel(8);

    connected_tx
      .send(data_tx)
      .expect("Failed to send connected signal");

    let mut buf = [0; PACKET_FRAME_SIZE];

    while let Ok(data) = data_rx.recv() {
      data.serialize(&mut buf);
      if perf!(
        "net write stream",
        write_stream(&mut stream, &buf).is_err(),
        10
      ) {
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

trait SerializablePacket {
  /// Serialize the packet into a buffer.
  fn serialize(&self, buf: &mut [u8; PACKET_FRAME_SIZE]);
}

impl<Gamepad: SerializableGamepad> SerializablePacket for Packet<Gamepad> {
  fn serialize(&self, buf: &mut [u8; PACKET_FRAME_SIZE]) {
    match self {
      Packet::Timestamp(timestamp) => {
        buf[0] = 0;
        buf[1..9].copy_from_slice(&timestamp.to_le_bytes());
      }
      Packet::Gamepad(gamepad) => {
        buf[0] = 1;
        gamepad.serialize(&mut buf[1..]);
      }
      Packet::Mouse(mouse) => {
        buf[0] = 2;
        mouse.serialize(&mut buf[1..]);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  stickdeck_common::impl_gamepad_tests! {XGamepad, XButtons}
}
