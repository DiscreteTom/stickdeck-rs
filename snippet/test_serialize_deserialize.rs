fn assert_serialize_deserialize(gamepad: &XGamepad) {
  let mut buf = [0; 12];
  gamepad.serialize(&mut buf);
  assert_eq!(gamepad, &XGamepad::deserialize(&buf));
}

#[test]
fn test_serialize_deserialize() {
  let mut gamepad = XGamepad::default();
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
