/// Macro to implement SerializableGamepad trait for XGamepad type.
/// This trait allows gamepad data to be serialized into a buffer.
#[macro_export]
macro_rules! impl_serializable_gamepad {
  ($XGamepad:ident) => {
    pub trait SerializableGamepad {
      /// Serialize the gamepad data into a buffer.
      /// The buffer must be at least 12 bytes long.
      fn serialize(&self, buf: &mut [u8]);
    }

    impl SerializableGamepad for $XGamepad {
      fn serialize(&self, buf: &mut [u8]) {
        debug_assert!(buf.len() >= 12);

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
