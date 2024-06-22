use super::action::{InputAnalogAction, InputDigitalAction};
use crate::utils::check_handle;
use steamworks::{ClientManager, Input};
use steamworks_sys::InputHandle_t;

pub struct XBoxControls {
  /// The action set handle.
  pub handle: InputHandle_t,
  // digital actions
  pub btn_up: InputDigitalAction,
  pub btn_down: InputDigitalAction,
  pub btn_left: InputDigitalAction,
  pub btn_right: InputDigitalAction,
  pub btn_start: InputDigitalAction,
  pub btn_back: InputDigitalAction,
  pub btn_l_thumb: InputDigitalAction,
  pub btn_r_thumb: InputDigitalAction,
  pub btn_lb: InputDigitalAction,
  pub btn_rb: InputDigitalAction,
  pub btn_a: InputDigitalAction,
  pub btn_b: InputDigitalAction,
  pub btn_x: InputDigitalAction,
  pub btn_y: InputDigitalAction,
  // analog actions
  pub lt: InputAnalogAction,
  pub rt: InputAnalogAction,
  pub l_move: InputAnalogAction,
  pub l_mouse: InputAnalogAction,
  pub r_move: InputAnalogAction,
  pub r_mouse: InputAnalogAction,
}

impl XBoxControls {
  /// Return `Ok` if all handles are valid.
  pub fn new(input: &Input<ClientManager>) -> Result<Self, ()> {
    Ok(Self {
      handle: check_handle(input.get_action_set_handle("XBoxControls"))?,

      btn_up: InputDigitalAction::new(input, "BtnUp")?,
      btn_down: InputDigitalAction::new(input, "BtnDown")?,
      btn_left: InputDigitalAction::new(input, "BtnLeft")?,
      btn_right: InputDigitalAction::new(input, "BtnRight")?,
      btn_start: InputDigitalAction::new(input, "BtnStart")?,
      btn_back: InputDigitalAction::new(input, "BtnBack")?,
      btn_l_thumb: InputDigitalAction::new(input, "BtnLeftThumb")?,
      btn_r_thumb: InputDigitalAction::new(input, "BtnRightThumb")?,
      btn_lb: InputDigitalAction::new(input, "BtnLB")?,
      btn_rb: InputDigitalAction::new(input, "BtnRB")?,
      btn_a: InputDigitalAction::new(input, "BtnA")?,
      btn_b: InputDigitalAction::new(input, "BtnB")?,
      btn_x: InputDigitalAction::new(input, "BtnX")?,
      btn_y: InputDigitalAction::new(input, "BtnY")?,

      lt: InputAnalogAction::new(input, "LeftTrigger")?,
      rt: InputAnalogAction::new(input, "RightTrigger")?,
      l_move: InputAnalogAction::new(input, "LeftMove")?,
      l_mouse: InputAnalogAction::new(input, "LeftMouse")?,
      r_move: InputAnalogAction::new(input, "RightMove")?,
      r_mouse: InputAnalogAction::new(input, "RightMouse")?,
    })
  }
}
