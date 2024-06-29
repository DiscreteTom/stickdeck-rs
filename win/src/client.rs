use log::{info, warn};
use std::{io::Read, net::TcpStream, sync::mpsc, thread};
use stickdeck_common::{MouseMove, Packet, PACKET_FRAME_SIZE};
use vigem_client::{XButtons, XGamepad};

pub fn spawn(server: &str, tx: mpsc::Sender<Packet<XGamepad>>) {
  info!("Connecting to the server...");
  let mut stream = TcpStream::connect(server).expect("Failed to connect to the server");
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

include!("../../snippet/deserialize.rs");

#[cfg(test)]
mod tests {
  use super::*;

  include!("../../snippet/serialize.rs");
  include!("../../snippet/test_serialize_deserialize.rs");
}
