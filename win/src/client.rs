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
      // println!("{:?}", std::time::SystemTime::now());
      // println!("{:?}", buf);

      tx.send(deserialize(&buf))
        .expect("Failed to send data to the main thread");
    }

    info!("Disconnected");
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
