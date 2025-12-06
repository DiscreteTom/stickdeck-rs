mod perf;

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

/// Macro to implement SerializableGamepad trait for XGamepad type.
/// This trait allows gamepad data to be serialized into a buffer.
#[macro_export]
macro_rules! impl_serializable_gamepad {
  ($XGamepad:ident, $XButtons:ident) => {
    pub trait SerializableGamepad {
      /// Serialize the gamepad data into a buffer.
      /// The buffer must be at least 12 bytes long.
      fn serialize(&self, buf: &mut [u8]);
    }

    impl SerializableGamepad for $XGamepad {
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
  };
}

/// Macro to implement DeserializableGamepad trait for XGamepad type.
/// This trait allows gamepad data to be deserialized from a buffer.
#[macro_export]
macro_rules! impl_deserializable_gamepad {
  ($XGamepad:ident, $XButtons:ident) => {
    pub trait DeserializableGamepad {
      type Target;
      /// Deserialize the gamepad data from a buffer.
      fn deserialize(buf: &[u8]) -> Self::Target;
    }

    impl DeserializableGamepad for $XGamepad {
      type Target = Self;

      fn deserialize(buf: &[u8]) -> Self {
        Self {
          buttons: $XButtons {
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
  };
}

/// Macro to generate test function for serialize/deserialize round-trip.
/// This tests that serializing and then deserializing produces the same gamepad state.
#[macro_export]
macro_rules! impl_test_serialize_deserialize {
  ($XGamepad:ident, $XButtons:ident) => {
    fn assert_serialize_deserialize(gamepad: &$XGamepad) {
      let mut buf = [0; 12];
      gamepad.serialize(&mut buf);
      assert_eq!(gamepad, &<$XGamepad>::deserialize(&buf));
    }

    #[test]
    fn test_serialize_deserialize() {
      let mut gamepad = <$XGamepad>::default();
      assert_serialize_deserialize(&gamepad);
      gamepad.buttons.raw = 0x1234;
      assert_serialize_deserialize(&gamepad);
      gamepad.left_trigger = 0x12;
      assert_serialize_deserialize(&gamepad);
      gamepad.right_trigger = 0x34;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_lx = 0x1234;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_ly = 0x5678;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_rx = -0x1234i16;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_ry = -0x5678i16;
      assert_serialize_deserialize(&gamepad);
    }
  };
}

/// Combined macro to implement both deserializable and tests in test modules
#[macro_export]
macro_rules! impl_gamepad_tests {
  ($XGamepad:ident, $XButtons:ident) => {
    pub trait DeserializableGamepad {
      type Target;
      /// Deserialize the gamepad data from a buffer.
      fn deserialize(buf: &[u8]) -> Self::Target;
    }

    impl DeserializableGamepad for $XGamepad {
      type Target = Self;

      fn deserialize(buf: &[u8]) -> Self {
        Self {
          buttons: $XButtons {
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

    fn assert_serialize_deserialize(gamepad: &$XGamepad) {
      let mut buf = [0; 12];
      gamepad.serialize(&mut buf);
      assert_eq!(gamepad, &<$XGamepad>::deserialize(&buf));
    }

    #[test]
    fn test_serialize_deserialize() {
      let mut gamepad = <$XGamepad>::default();
      assert_serialize_deserialize(&gamepad);
      gamepad.buttons.raw = 0x1234;
      assert_serialize_deserialize(&gamepad);
      gamepad.left_trigger = 0x12;
      assert_serialize_deserialize(&gamepad);
      gamepad.right_trigger = 0x34;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_lx = 0x1234;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_ly = 0x5678;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_rx = -0x1234i16;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_ry = -0x5678i16;
      assert_serialize_deserialize(&gamepad);
    }
  };
}

/// Combined macro for client-side tests (serializable + tests)
#[macro_export]
macro_rules! impl_client_gamepad_tests {
  ($XGamepad:ident, $XButtons:ident) => {
    pub trait SerializableGamepad {
      /// Serialize the gamepad data into a buffer.
      /// The buffer must be at least 12 bytes long.
      fn serialize(&self, buf: &mut [u8]);
    }

    impl SerializableGamepad for $XGamepad {
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

    fn assert_serialize_deserialize(gamepad: &$XGamepad) {
      let mut buf = [0; 12];
      gamepad.serialize(&mut buf);
      assert_eq!(gamepad, &<$XGamepad>::deserialize(&buf));
    }

    #[test]
    fn test_serialize_deserialize() {
      let mut gamepad = <$XGamepad>::default();
      assert_serialize_deserialize(&gamepad);
      gamepad.buttons.raw = 0x1234;
      assert_serialize_deserialize(&gamepad);
      gamepad.left_trigger = 0x12;
      assert_serialize_deserialize(&gamepad);
      gamepad.right_trigger = 0x34;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_lx = 0x1234;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_ly = 0x5678;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_rx = -0x1234i16;
      assert_serialize_deserialize(&gamepad);
      gamepad.thumb_ry = -0x5678i16;
      assert_serialize_deserialize(&gamepad);
    }
  };
}
