use log::{info, warn};
use std::{io::Read, net::TcpStream, sync::mpsc, thread};
use stickdeck_common::{Mouse, Packet, PACKET_FRAME_SIZE};
use vigem_client::{XButtons, XGamepad};

pub fn spawn(server: &str, tx: mpsc::Sender<Packet<XGamepad>>) {
  info!("Connecting to {} ...", server);
  let mut retry = 3;
  let mut stream = loop {
    if retry == 0 {
      panic!("Failed to connect to the server: retry limit exceeded");
    }
    if let Ok(stream) = TcpStream::connect(server) {
      break stream;
    }
    info!("Failed to connect to the server: retrying ...");
    retry -= 1;
  };
  info!("Connected");

  thread::spawn(move || {
    let mut buf = [0; PACKET_FRAME_SIZE];
    while let Ok(_) = stream.read_exact(&mut buf) {
      match Packet::deserialize(&buf) {
        Ok(packet) => {
          tx.send(packet)
            .expect("Failed to send data to the main thread");
        }
        Err(_) => {
          warn!("Invalid packet: {:?}", buf);
        }
      }
    }

    info!("Disconnected");
  });
}

trait DeserializablePacket {
  type Target;
  /// Deserialize the packet from a buffer.
  /// Return the deserialized packet.
  fn deserialize(buf: &[u8; PACKET_FRAME_SIZE]) -> Result<Self::Target, u8>;
}

impl<Gamepad: DeserializableGamepad<Target = Gamepad>> DeserializablePacket for Packet<Gamepad> {
  type Target = Self;

  /// Deserialize the packet from a buffer.
  /// Return the deserialized packet.
  fn deserialize(buf: &[u8; PACKET_FRAME_SIZE]) -> Result<Self, u8> {
    match buf[0] {
      0 => {
        let timestamp = u64::from_le_bytes(buf[1..9].try_into().unwrap());
        Ok(Packet::Timestamp(timestamp))
      }
      1 => Ok(Packet::Gamepad(Gamepad::deserialize(&buf[1..]))),
      2 => Ok(Packet::Mouse(Mouse::deserialize(&buf[1..]))),
      3 => Ok(Packet::GamepadAndMouse(
        Gamepad::deserialize(&buf[1..]),
        Mouse::deserialize(&buf[13..]),
      )),
      _ => Err(buf[0]),
    }
  }
}

include!("../../snippet/deserialize.rs");

#[cfg(test)]
mod tests {
  use super::*;

  include!("../../snippet/serialize.rs");
  include!("../../snippet/test_serialize_deserialize.rs");
}
