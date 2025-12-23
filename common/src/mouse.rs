#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButton(pub u8);

impl MouseButton {
  pub const MOUSE_LEFT_BUTTON: u8 = 1;
  pub const MOUSE_RIGHT_BUTTON: u8 = 2;

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
    debug_assert!(buf.len() >= 4);

    buf[0] = self.x as u8;
    buf[1] = self.y as u8;
    buf[2] = self.buttons.0;
    buf[3] = self.scroll as u8;
  }

  /// Deserialize the mouse movement data from a buffer.
  /// The buffer must be at least 4 bytes long.
  pub fn deserialize(buf: &[u8]) -> Self {
    debug_assert!(buf.len() >= 4);
    
    Self {
      x: buf[0] as i8,
      y: buf[1] as i8,
      buttons: MouseButton(buf[2]),
      scroll: buf[3] as i8,
    }
  }
}
