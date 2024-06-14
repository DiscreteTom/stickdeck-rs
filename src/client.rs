use crate::serde::serialize;
use std::{
  io::Write,
  net::{SocketAddr, TcpStream},
  sync::mpsc,
  thread,
};
use vigem_client::XGamepad;

pub fn spawn(server: SocketAddr, rx: mpsc::Receiver<XGamepad>) {
  let mut stream = TcpStream::connect(server).expect("Failed to connect to the server");
  stream.set_nodelay(true).expect("Failed to set nodelay");

  thread::spawn(move || loop {
    let data = rx.recv().expect("Failed to receive data from the UI");
    stream
      .write_all(&serialize(&data))
      .expect("Failed to send data to the server");
    stream.flush().expect("Failed to flush the stream");
  });
}
