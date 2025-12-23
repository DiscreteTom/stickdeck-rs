/// Macro to implement DeserializableGamepad trait for XGamepad type.
/// This trait allows gamepad data to be deserialized from a buffer.
#[macro_export]
macro_rules! impl_deserializable_gamepad {
  ($XGamepad:ident, $XButtons:ident) => {
    pub trait DeserializableGamepad {
      type Target;
      /// Deserialize the gamepad data from a buffer.
      /// The buffer must be at least 12 bytes long.
      fn deserialize(buf: &[u8]) -> Self::Target;
    }

    impl DeserializableGamepad for $XGamepad {
      type Target = Self;

      fn deserialize(buf: &[u8]) -> Self {
        debug_assert!(buf.len() >= 12);

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
