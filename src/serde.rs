use vigem_client::{XButtons, XGamepad};

// TODO: use mem::transmute to serialize/deserialize the struct?

pub fn serialize(gamepad: &XGamepad) -> [u8; 12] {
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

pub fn deserialize(buf: &[u8; 12]) -> XGamepad {
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
