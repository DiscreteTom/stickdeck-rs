// copied from https://github.com/CasualX/vigem-client/blob/acbc38bbc159e315622b5dbd9b41048fc9f3164c/src/x360.rs

#[derive(Default, Clone)]
pub struct XButtons {
  pub raw: u16,
}

impl XButtons {
  /// Dpad up button.
  pub const UP: u16 = 0x0001;
  /// Dpad down button.
  pub const DOWN: u16 = 0x0002;
  /// Dpad left button.
  pub const LEFT: u16 = 0x0004;
  /// Dpad right button.
  pub const RIGHT: u16 = 0x0008;
  /// Start button.
  pub const START: u16 = 0x0010;
  /// Back button.
  pub const BACK: u16 = 0x0020;
  /// Left thumb button.
  pub const LTHUMB: u16 = 0x0040;
  /// Right thumb button.
  pub const RTHUMB: u16 = 0x0080;
  /// Left shoulder button.
  pub const LB: u16 = 0x0100;
  /// Right shoulder button.
  pub const RB: u16 = 0x0200;
  /// Xbox guide button.
  #[allow(dead_code)]
  pub const GUIDE: u16 = 0x0400;
  /// A button.
  pub const A: u16 = 0x1000;
  /// B button.
  pub const B: u16 = 0x2000;
  /// X button.
  pub const X: u16 = 0x4000;
  /// Y button.
  pub const Y: u16 = 0x8000;
}

#[derive(Default, Clone)]
pub struct XGamepad {
  pub buttons: XButtons,
  pub left_trigger: u8,
  pub right_trigger: u8,
  pub thumb_lx: i16,
  pub thumb_ly: i16,
  pub thumb_rx: i16,
  pub thumb_ry: i16,
}
