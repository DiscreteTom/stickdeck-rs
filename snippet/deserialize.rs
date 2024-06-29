// since the `XGamePad` in the client side and the server side are two different struct,
// we use `include!` to include this code snippet in both client and server side
// to keep the behaviour consistent.

pub trait DeserializableGamepad {
  type Target;
  /// Deserialize the gamepad data from a buffer.
  fn deserialize(buf: &[u8]) -> Self::Target;
}

impl DeserializableGamepad for XGamepad {
  type Target = Self;

  fn deserialize(buf: &[u8]) -> Self {
    Self {
      buttons: XButtons {
        raw: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
      },
      left_trigger: buf[2],
      right_trigger: buf[3],
      thumb_lx: i16::from_le_bytes(buf[4..6].try_into().unwrap()),
      thumb_ly: i16::from_le_bytes(buf[6..8].try_into().unwrap()),
      thumb_rx: i16::from_le_bytes(buf[8..10].try_into().unwrap()),
      thumb_ry: i16::from_le_bytes(buf[10..12].try_into().unwrap()),
    }
  }
}
