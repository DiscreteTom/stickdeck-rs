// since the `XGamePad` in the client side and the server side are two different struct,
// we use `include!` to include this code snippet in both client and server side.

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

pub trait DeserializablePacket {
  type Target;
  /// Deserialize the packet from a buffer.
  /// Return the deserialized packet.
  fn deserialize(buf: &[u8; PACKET_FRAME_SIZE]) -> Result<Self::Target, u8>;
}

impl<Gamepad: DeserializableGamepad<Target = Gamepad>> DeserializablePacket for Packet<Gamepad> {
  type Target = Self;

  /// Deserialize the packet from a buffer.
  /// Return the deserialized packet.
  fn deserialize(buf: &[u8; PACKET_FRAME_SIZE]) -> Result<Self, u8> {
    match buf[0] {
      0 => {
        let timestamp = u64::from_le_bytes(buf[1..9].try_into().unwrap());
        Ok(Packet::Timestamp(timestamp))
      }
      1 => Ok(Packet::Gamepad(Gamepad::deserialize(&buf[1..]))),
      2 => Ok(Packet::MouseMove(MouseMove::deserialize(&buf[1..]))),
      3 => Ok(Packet::GamepadAndMouseMove(
        Gamepad::deserialize(&buf[1..]),
        MouseMove::deserialize(&buf[13..]),
      )),
      _ => Err(buf[0]),
    }
  }
}
