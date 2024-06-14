use crate::serde::deserialize;
use std::{
  io::Read,
  net::{SocketAddr, TcpListener, TcpStream},
  sync::mpsc,
  thread,
};
use vigem_client::XGamepad;

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<XGamepad>) {
  let mut buf = [0; 12];

  loop {
    stream
      .read_exact(&mut buf)
      .expect("Failed to read data from the client");

    // println!("{:?}", std::time::SystemTime::now());

    tx.send(deserialize(&buf))
      .expect("Failed to send data to the main thread");
  }
}

pub fn spawn(port: u16, tx: mpsc::Sender<XGamepad>) {
  let listener =
    TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).expect("Failed to bind to address");

  println!("Server listening on port {}", port);

  thread::spawn(move || {
    for stream in listener.incoming() {
      let stream = stream.expect("Failed to accept connection");
      let tx = tx.clone();
      println!("New client connected");
      thread::spawn(move || handle_client(stream, tx));
    }
  });
}
