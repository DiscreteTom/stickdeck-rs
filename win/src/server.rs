use std::{
  io::Read,
  net::{SocketAddr, TcpListener, TcpStream},
  sync::mpsc,
  thread,
};
use vigem_client::{XButtons, XGamepad};

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<XGamepad>) {
  let mut buf = [0; 12];

  while let Ok(_) = stream.read_exact(&mut buf) {
    // println!("{:?}", std::time::SystemTime::now());
    // println!("{:?}", buf);

    tx.send(deserialize(&buf))
      .expect("Failed to send data to the main thread");
  }

  println!("Client disconnected");
}

pub fn spawn(addr: SocketAddr, tx: mpsc::Sender<XGamepad>) {
  let listener = TcpListener::bind(addr).expect(&format!("Failed to bind to address {}", addr));

  println!("Server listening on {}", addr);

  thread::spawn(move || {
    for stream in listener.incoming() {
      let stream = stream.expect("Failed to accept connection");
      let tx = tx.clone();
      println!("New client connected");
      thread::spawn(move || handle_client(stream, tx));
    }
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
