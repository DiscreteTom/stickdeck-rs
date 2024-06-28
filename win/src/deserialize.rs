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
