use std::{
  io::Read,
  net::{SocketAddr, TcpListener, TcpStream},
  sync::mpsc,
  thread,
};
use vigem_client::{XButtons, XGamepad};

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<XGamepad>) {
  let mut buf = [0; 12];

  loop {
    stream
      .read_exact(&mut buf)
      .expect("Failed to read data from the client");

    // TODO: use mem::transmute to convert the buffer to XGamepad
    let gamepad = XGamepad {
      buttons: XButtons {
        raw: u16::from_le_bytes([buf[0], buf[1]]),
      },
      left_trigger: buf[2],
      right_trigger: buf[3],
      thumb_lx: i16::from_le_bytes([buf[4], buf[5]]),
      thumb_ly: i16::from_le_bytes([buf[6], buf[7]]),
      thumb_rx: i16::from_le_bytes([buf[8], buf[9]]),
      thumb_ry: i16::from_le_bytes([buf[10], buf[11]]),
    };

    tx.send(gamepad)
      .expect("Failed to send data to the main thread");
  }
}

pub fn start(port: u16, tx: mpsc::Sender<XGamepad>) {
  let listener =
    TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).expect("Failed to bind to address");

  for stream in listener.incoming() {
    let stream = stream.expect("Failed to accept connection");
    let tx = tx.clone();
    thread::spawn(move || handle_client(stream, tx));
  }
}
