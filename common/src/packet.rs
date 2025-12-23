use crate::mouse::Mouse;

#[derive(Debug)]
pub enum Packet<Gamepad> {
  Timestamp(u64),
  Gamepad(Gamepad),
  Mouse(Mouse),
}

pub const PACKET_FRAME_SIZE: usize = 16;
