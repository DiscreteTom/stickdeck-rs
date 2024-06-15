use local_ip_address::local_ip;
use std::{net::UdpSocket, sync::mpsc, thread};
use vigem_client::{XButtons, XGamepad};

pub fn spawn(server: &str, tx: mpsc::Sender<XGamepad>) {
  let socket = UdpSocket::bind(format!(
    "{}:0",
    // bind to local address, let the OS choose the port
    local_ip().expect("Failed to get local ip address")
  ))
  .expect("Failed to bind to the address");
  socket
    .connect(server)
    .expect("Failed to connect to the server");

  // send a message to report the client's address to the server
  socket
    .send(&[0])
    .expect("Failed to send a message to the server");
  println!("Connected.");

  thread::spawn(move || {
    let mut buf = [0; 12];

    while let Ok(_) = socket.recv(&mut buf) {
      // println!("{:?}", std::time::SystemTime::now());
      // println!("{:?}", buf);

      tx.send(deserialize(&buf))
        .expect("Failed to send data to the main thread");
    }

    println!("Disconnected.");
  });
}

fn deserialize(buf: &[u8; 12]) -> XGamepad {
  XGamepad {
    buttons: XButtons {
      raw: u16::from_le_bytes([buf[0], buf[1]]),
    },
    left_trigger: buf[2],
    right_trigger: buf[3],
    thumb_lx: i16::from_le_bytes([buf[4], buf[5]]),
    thumb_ly: i16::from_le_bytes([buf[6], buf[7]]),
    thumb_rx: i16::from_le_bytes([buf[8], buf[9]]),
    thumb_ry: i16::from_le_bytes([buf[10], buf[11]]),
  }
}
