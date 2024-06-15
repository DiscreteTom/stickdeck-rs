use super::input_action::{InputAnalogAction, InputDigitalAction};
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

      btn_up: InputDigitalAction::new(input, "btn_up")?,
      btn_down: InputDigitalAction::new(input, "btn_down")?,
      btn_left: InputDigitalAction::new(input, "btn_left")?,
      btn_right: InputDigitalAction::new(input, "btn_right")?,
      btn_start: InputDigitalAction::new(input, "btn_start")?,
      btn_back: InputDigitalAction::new(input, "btn_back")?,
      btn_l_thumb: InputDigitalAction::new(input, "btn_l_thumb")?,
      btn_r_thumb: InputDigitalAction::new(input, "btn_r_thumb")?,
      btn_lb: InputDigitalAction::new(input, "btn_lb")?,
      btn_rb: InputDigitalAction::new(input, "btn_rb")?,
      btn_a: InputDigitalAction::new(input, "btn_a")?,
      btn_b: InputDigitalAction::new(input, "btn_b")?,
      btn_x: InputDigitalAction::new(input, "btn_x")?,
      btn_y: InputDigitalAction::new(input, "btn_y")?,

      lt: InputAnalogAction::new(input, "left_trigger")?,
      rt: InputAnalogAction::new(input, "right_trigger")?,
      l_move: InputAnalogAction::new(input, "LeftMove")?, // TODO: unify name convention?
      l_mouse: InputAnalogAction::new(input, "LeftMouse")?,
      r_move: InputAnalogAction::new(input, "RightMove")?,
      r_mouse: InputAnalogAction::new(input, "RightMouse")?,
    })
  }
}
