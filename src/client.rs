use crate::serde::serialize;
use std::{
  io::Write,
  net::{SocketAddr, TcpStream},
  sync::mpsc,
};
use vigem_client::XGamepad;

pub fn start(server: SocketAddr, rx: mpsc::Receiver<XGamepad>) {
  let mut stream = TcpStream::connect(server).expect("Failed to connect to the server");

  loop {
    let data = rx.recv().expect("Failed to receive data from the UI");
    stream
      .write_all(&serialize(&data))
      .expect("Failed to send data to the server");
  }
}
