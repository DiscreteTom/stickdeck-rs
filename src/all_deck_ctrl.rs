use super::action_repo::{
  ActionRepo, AnalogAction, DigitalAction, WrappedAnalogAction, WrappedDigitalAction,
};
use crate::action_repo::check_handle;
use steamworks::{ClientManager, Input};
use steamworks_sys::InputHandle_t;

pub struct AllDeckControls {
  /// The action set handle.
  pub handle: InputHandle_t,
  // actions
  pub repo: ActionRepo,
  pub btn_a: WrappedDigitalAction,
  pub btn_b: WrappedDigitalAction,
  pub btn_x: WrappedDigitalAction,
  pub btn_y: WrappedDigitalAction,
  pub btn_lb: WrappedDigitalAction,
  pub btn_rb: WrappedDigitalAction,
  pub btn_l4: WrappedDigitalAction,
  pub btn_r4: WrappedDigitalAction,
  pub btn_l5: WrappedDigitalAction,
  pub btn_r5: WrappedDigitalAction,
  pub btn_start: WrappedDigitalAction,
  pub btn_select: WrappedDigitalAction,
  pub lt: WrappedAnalogAction,
  pub rt: WrappedAnalogAction,
  pub move1: WrappedAnalogAction,
  pub move2: WrappedAnalogAction,
  pub move3: WrappedAnalogAction,
  pub mouse1: WrappedAnalogAction,
  pub mouse2: WrappedAnalogAction,
  pub mouse3: WrappedAnalogAction,
}

impl AllDeckControls {
  /// Return `Ok` if all handles are valid.
  pub fn new(input: &Input<ClientManager>) -> Result<Self, ()> {
    let mut repo = ActionRepo::new();
    Ok(Self {
      handle: check_handle(input.get_action_set_handle("AllDeckControls"))?,
      btn_a: repo.append(DigitalAction::new(input, "btn_a")?),
      btn_b: repo.append(DigitalAction::new(input, "btn_b")?),
      btn_x: repo.append(DigitalAction::new(input, "btn_x")?),
      btn_y: repo.append(DigitalAction::new(input, "btn_y")?),
      btn_lb: repo.append(DigitalAction::new(input, "btn_lb")?),
      btn_rb: repo.append(DigitalAction::new(input, "btn_rb")?),
      btn_l4: repo.append(DigitalAction::new(input, "btn_l4")?),
      btn_r4: repo.append(DigitalAction::new(input, "btn_r4")?),
      btn_l5: repo.append(DigitalAction::new(input, "btn_l5")?),
      btn_r5: repo.append(DigitalAction::new(input, "btn_r5")?),
      btn_start: repo.append(DigitalAction::new(input, "btn_start")?),
      btn_select: repo.append(DigitalAction::new(input, "btn_select")?),
      lt: repo.append(AnalogAction::new(input, "left_trigger")?),
      rt: repo.append(AnalogAction::new(input, "right_trigger")?),
      move1: repo.append(AnalogAction::new(input, "Move1")?),
      move2: repo.append(AnalogAction::new(input, "Move2")?),
      move3: repo.append(AnalogAction::new(input, "Move3")?),
      mouse1: repo.append(AnalogAction::new(input, "Mouse1")?),
      mouse2: repo.append(AnalogAction::new(input, "Mouse2")?),
      mouse3: repo.append(AnalogAction::new(input, "Mouse3")?),
      repo,
    })
  }

  pub fn update(&mut self, input: &Input<ClientManager>, input_handle: InputHandle_t) {
    self.repo.update(input, input_handle);
  }
}
