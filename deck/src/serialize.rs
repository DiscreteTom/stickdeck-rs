// since the `XGamePad` in the client side and the server side are two different struct,
// we use `include!` to include this code snippet in both client and server side
// to keep the behaviour consistent.

pub trait SerializableGamepad {
  /// Serialize the gamepad data into a buffer.
  /// The buffer must be at least 12 bytes long.
  fn serialize(&self, buf: &mut [u8]);
}

impl SerializableGamepad for XGamepad {
  fn serialize(&self, buf: &mut [u8]) {
    buf[0..2].copy_from_slice(&self.buttons.raw.to_le_bytes());
    buf[2] = self.left_trigger;
    buf[3] = self.right_trigger;
    buf[4..6].copy_from_slice(&self.thumb_lx.to_le_bytes());
    buf[6..8].copy_from_slice(&self.thumb_ly.to_le_bytes());
    buf[8..10].copy_from_slice(&self.thumb_rx.to_le_bytes());
    buf[10..12].copy_from_slice(&self.thumb_ry.to_le_bytes());
  }
}

pub trait SerializablePacket {
  /// Serialize the packet into a buffer.
  fn serialize(&self, buf: &mut [u8; PACKET_FRAME_SIZE]);
}

impl<Gamepad: SerializableGamepad> SerializablePacket for Packet<Gamepad> {
  fn serialize(&self, buf: &mut [u8; PACKET_FRAME_SIZE]) {
    match self {
      Packet::Timestamp(timestamp) => {
        buf[0] = 0;
        buf[1..9].copy_from_slice(&timestamp.to_le_bytes());
      }
      Packet::GamePad(gamepad) => {
        buf[0] = 1;
        gamepad.serialize(&mut buf[1..]);
      }
      Packet::MouseMove(mouse) => {
        buf[0] = 2;
        mouse.serialize(&mut buf[1..]);
      }
      Packet::GamePadAndMouseMove(gamepad, mouse) => {
        buf[0] = 3;
        gamepad.serialize(&mut buf[1..]);
        mouse.serialize(&mut buf[13..]);
      }
    }
  }
}
