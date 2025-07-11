mod perf;

pub const PORT: u16 = 7777;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButton(pub u8);

impl MouseButton {
  pub const MOUSE_LEFT_BUTTON: u8 = 1;
  pub const MOUSE_RIGHT_BUTTON: u8 = 2;
  pub const MOUSE_MIDDLE_BUTTON: u8 = 4;

  pub fn left_button_down(&mut self) {
    self.0 |= Self::MOUSE_LEFT_BUTTON;
  }
  pub fn is_left_button_down(&self) -> bool {
    self.0 & Self::MOUSE_LEFT_BUTTON != 0
  }
  pub fn right_button_down(&mut self) {
    self.0 |= Self::MOUSE_RIGHT_BUTTON;
  }
  pub fn is_right_button_down(&self) -> bool {
    self.0 & Self::MOUSE_RIGHT_BUTTON != 0
  }
  
  pub const LEFT: Self = Self(Self::MOUSE_LEFT_BUTTON);
  pub const RIGHT: Self = Self(Self::MOUSE_RIGHT_BUTTON);
  pub const MIDDLE: Self = Self(Self::MOUSE_MIDDLE_BUTTON);
  
  pub const fn empty() -> Self {
    Self(0)
  }
  
  pub fn contains(&self, other: Self) -> bool {
    self.0 & other.0 != 0
  }
}

/// The mouse movement data in pixels in one update.
#[derive(Default, Debug, Clone, Copy)]
pub struct Mouse {
  pub x: i8,
  pub y: i8,
  pub buttons: MouseButton,
  pub scroll: i8,
}

impl Mouse {
  /// Serialize the mouse movement data into a buffer.
  /// The buffer must be at least 4 bytes long.
  pub fn serialize(&self, buf: &mut [u8]) {
    buf[0] = self.x as u8;
    buf[1] = self.y as u8;
    buf[2] = self.buttons.0;
    buf[3] = self.scroll as u8;
  }

  /// Deserialize the mouse movement data from a buffer.
  /// The buffer must be at least 4 bytes long.
  pub fn deserialize(buf: &[u8]) -> Self {
    Self {
      x: buf[0] as i8,
      y: buf[1] as i8,
      buttons: MouseButton(buf[2]),
      scroll: buf[3] as i8,
    }
  }
}

#[derive(Debug)]
pub enum Packet<Gamepad> {
  Timestamp(u64),
  Gamepad(Gamepad),
  Mouse(Mouse),
}

pub const PACKET_FRAME_SIZE: usize = 16;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct XButtons {
  pub raw: u16,
}

impl XButtons {
  pub const DPAD_UP: u16 = 0x0001;
  pub const DPAD_DOWN: u16 = 0x0002;
  pub const DPAD_LEFT: u16 = 0x0004;
  pub const DPAD_RIGHT: u16 = 0x0008;
  pub const START: u16 = 0x0010;
  pub const BACK: u16 = 0x0020;
  pub const LEFT_THUMB: u16 = 0x0040;
  pub const RIGHT_THUMB: u16 = 0x0080;
  pub const LEFT_SHOULDER: u16 = 0x0100;
  pub const RIGHT_SHOULDER: u16 = 0x0200;
  pub const GUIDE: u16 = 0x0400;
  pub const A: u16 = 0x1000;
  pub const B: u16 = 0x2000;
  pub const X: u16 = 0x4000;
  pub const Y: u16 = 0x8000;
  
  pub const fn empty() -> Self {
    Self { raw: 0 }
  }
  
  pub fn contains(&self, button: u16) -> bool {
    self.raw & button != 0
  }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct XGamepad {
  pub buttons: XButtons,
  pub left_trigger: u8,
  pub right_trigger: u8,
  pub thumb_lx: i16,
  pub thumb_ly: i16,
  pub thumb_rx: i16,
  pub thumb_ry: i16,
}
