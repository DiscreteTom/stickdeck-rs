use crate::gamepad::XGamepad;
use std::{net::UdpSocket, sync::mpsc, thread};

pub fn spawn(addr: &str, rx: mpsc::Receiver<XGamepad>) {
  let socket = UdpSocket::bind(addr).expect(&format!("Failed to bind to address {}", addr));
  // set non blocking so we can poll the socket
  socket
    .set_nonblocking(true)
    .expect("Failed to set non-blocking mode");
  println!("Server listening on {}", addr);

  thread::spawn(move || {
    let mut connected = false;
    let mut buf = [0; 1];
    while let Ok(data) = rx.recv() {
      if !connected {
        // poll the socket to try to get the client address
        socket.recv_from(&mut buf).ok().map(|(_, addr)| {
          socket
            .connect(addr)
            .expect("Failed to connect to the client");
          connected = true
        });
      }

      if connected {
        if socket.send(&serialize(&data)).is_err() {
          break;
        }
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
