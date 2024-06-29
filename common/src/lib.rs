/// The mouse movement data in pixels in one update.
#[derive(Default, Debug, Clone, Copy)]
pub struct MouseMove {
  pub x: i8,
  pub y: i8,
}

impl MouseMove {
  /// Serialize the mouse movement data into a buffer.
  /// The buffer must be at least 2 bytes long.
  pub fn serialize(&self, buf: &mut [u8]) {
    buf[0] = self.x as u8;
    buf[1] = self.y as u8;
  }

  /// Deserialize the mouse movement data from a buffer.
  /// The buffer must be at least 2 bytes long.
  pub fn deserialize(buf: &[u8]) -> Self {
    Self {
      x: buf[0] as i8,
      y: buf[1] as i8,
    }
  }
}

#[derive(Debug)]
pub enum Packet<Gamepad> {
  Timestamp(u64),
  Gamepad(Gamepad),
  MouseMove(MouseMove),
  GamepadAndMouseMove(Gamepad, MouseMove),
}

pub const PACKET_FRAME_SIZE: usize = 16;
