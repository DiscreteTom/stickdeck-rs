fn serialize(gamepad: &XGamepad) -> [u8; 12] {
  let mut buf = [0; 12];

  buf[0..2].copy_from_slice(&gamepad.buttons.raw.to_le_bytes());
  buf[2] = gamepad.left_trigger;
  buf[3] = gamepad.right_trigger;
  buf[4..6].copy_from_slice(&gamepad.thumb_lx.to_le_bytes());
  buf[6..8].copy_from_slice(&gamepad.thumb_ly.to_le_bytes());
  buf[8..10].copy_from_slice(&gamepad.thumb_rx.to_le_bytes());
  buf[10..12].copy_from_slice(&gamepad.thumb_ry.to_le_bytes());

  buf
}
