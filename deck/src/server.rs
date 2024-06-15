use crate::gamepad::XGamepad;
use std::{io::Write, net::TcpListener, sync::mpsc, thread};

pub fn spawn(addr: &str, rx: mpsc::Receiver<XGamepad>) {
  let listener = TcpListener::bind(addr).expect(&format!("Failed to bind to address {}", addr));

  println!("Server listening on {}", addr);

  thread::spawn(move || {
    // only accept one client because we will consume the receiver
    let mut stream = listener
      .incoming()
      .next()
      .unwrap()
      .expect("Failed to accept connection");
    stream.set_nodelay(true).expect("Failed to set nodelay");
    println!("New client connected");

    // TODO: drop data before connected
    while let Ok(data) = rx.recv() {
      if stream
        .write_all(&serialize(&data))
        .and_then(|_| stream.flush())
        .is_err()
      {
        break;
      }
    }

    println!("Client disconnected");
  });
}

fn serialize(gamepad: &XGamepad) -> [u8; 12] {
  let mut buf = [0; 12];

  buf[0..2].copy_from_slice(&gamepad.buttons.raw.to_le_bytes());
  buf[2] = gamepad.left_trigger;
  buf[3] = gamepad.right_trigger;
  buf[4..6].copy_from_slice(&gamepad.thumb_lx.to_le_bytes());
  buf[6..8].copy_from_slice(&gamepad.thumb_ly.to_le_bytes());
  buf[8..10].copy_from_slice(&gamepad.thumb_rx.to_le_bytes());
  buf[10..12].copy_from_slice(&gamepad.thumb_ry.to_le_bytes());

  buf
}
