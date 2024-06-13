use super::input_action::{AnalogAction, DigitalAction};
use crate::utils::check_handle;
use steamworks::{ClientManager, Input};
use steamworks_sys::InputHandle_t;

pub struct XBoxControls {
  /// The action set handle.
  pub handle: InputHandle_t,
  // digital actions
  pub btn_up: DigitalAction,
  pub btn_down: DigitalAction,
  pub btn_left: DigitalAction,
  pub btn_right: DigitalAction,
  pub btn_start: DigitalAction,
  pub btn_back: DigitalAction,
  pub btn_l_thumb: DigitalAction,
  pub btn_r_thumb: DigitalAction,
  pub btn_lb: DigitalAction,
  pub btn_rb: DigitalAction,
  pub btn_a: DigitalAction,
  pub btn_b: DigitalAction,
  pub btn_x: DigitalAction,
  pub btn_y: DigitalAction,
  // analog actions
  pub lt: AnalogAction,
  pub rt: AnalogAction,
  pub l_move: AnalogAction,
  pub l_mouse: AnalogAction,
  pub r_move: AnalogAction,
  pub r_mouse: AnalogAction,
}

impl XBoxControls {
  /// Return `Ok` if all handles are valid.
  pub fn new(input: &Input<ClientManager>) -> Result<Self, ()> {
    Ok(Self {
      handle: check_handle(input.get_action_set_handle("XBoxControls"))?,

      btn_up: DigitalAction::new(input, "btn_up")?,
      btn_down: DigitalAction::new(input, "btn_down")?,
      btn_left: DigitalAction::new(input, "btn_left")?,
      btn_right: DigitalAction::new(input, "btn_right")?,
      btn_start: DigitalAction::new(input, "btn_start")?,
      btn_back: DigitalAction::new(input, "btn_back")?,
      btn_l_thumb: DigitalAction::new(input, "btn_l_thumb")?,
      btn_r_thumb: DigitalAction::new(input, "btn_r_thumb")?,
      btn_lb: DigitalAction::new(input, "btn_lb")?,
      btn_rb: DigitalAction::new(input, "btn_rb")?,
      btn_a: DigitalAction::new(input, "btn_a")?,
      btn_b: DigitalAction::new(input, "btn_b")?,
      btn_x: DigitalAction::new(input, "btn_x")?,
      btn_y: DigitalAction::new(input, "btn_y")?,

      lt: AnalogAction::new(input, "left_trigger")?,
      rt: AnalogAction::new(input, "right_trigger")?,
      l_move: AnalogAction::new(input, "LeftMove")?,
      l_mouse: AnalogAction::new(input, "LeftMouse")?,
      r_move: AnalogAction::new(input, "RightMove")?,
      r_mouse: AnalogAction::new(input, "RightMouse")?,
    })
  }
}
