use log::info;
use std::{io::Read, net::TcpStream, sync::mpsc, thread};
use vigem_client::{XButtons, XGamepad};

pub fn spawn(server: &str, tx: mpsc::Sender<XGamepad>) {
  info!("Connecting to the server...");
  let mut stream = TcpStream::connect(server).expect("Failed to connect to the server");
  info!("Connected");

  thread::spawn(move || {
    let mut buf = [0; 12];

    while let Ok(_) = stream.read_exact(&mut buf) {
      tx.send(deserialize(&buf))
        .expect("Failed to send data to the main thread");
    }

    info!("Disconnected");
  });
}

include!("deserialize.rs");

#[cfg(test)]
mod tests {
  use super::*;

  include!("../../deck/src/serialize.rs");
  include!("../../snippet/test_serialize_deserialize.rs");
}
