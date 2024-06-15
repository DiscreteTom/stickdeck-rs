use crate::utils::check_handle;
use std::marker::PhantomData;
use steamworks::{ClientManager, Input};
use steamworks_sys::{uint64, InputAnalogActionData_t, InputDigitalActionData_t, InputHandle_t};

pub struct InputAction<Data> {
  pub name: &'static str,
  pub handle: uint64,
  _phantom: PhantomData<Data>,
}

pub type InputAnalogAction = InputAction<InputAnalogActionData_t>;
pub type InputDigitalAction = InputAction<InputDigitalActionData_t>;

impl InputAnalogAction {
  /// Create a new analog action.
  /// Return [`Err`] if the handle is invalid.
  pub fn new(input: &Input<ClientManager>, name: &'static str) -> Result<Self, ()> {
    Ok(Self {
      name,
      handle: check_handle(input.get_analog_action_handle(name))?,
      _phantom: PhantomData,
    })
  }
}

impl InputDigitalAction {
  /// Create a new digital action.
  /// Return [`Err`] if the handle is invalid.
  pub fn new(input: &Input<ClientManager>, name: &'static str) -> Result<Self, ()> {
    Ok(Self {
      name,
      handle: check_handle(input.get_digital_action_handle(name))?,
      _phantom: PhantomData,
    })
  }
}

pub trait UpdatableInputAction<Data> {
  /// Retrieve the input action's latest data.
  fn update(&self, input: &Input<ClientManager>, input_handle: InputHandle_t) -> Data;
}

impl UpdatableInputAction<InputAnalogActionData_t> for InputAnalogAction {
  fn update(
    &self,
    input: &Input<ClientManager>,
    input_handle: InputHandle_t,
  ) -> InputAnalogActionData_t {
    input.get_analog_action_data(input_handle, self.handle)
  }
}

impl UpdatableInputAction<InputDigitalActionData_t> for InputDigitalAction {
  fn update(
    &self,
    input: &Input<ClientManager>,
    input_handle: InputHandle_t,
  ) -> InputDigitalActionData_t {
    input.get_digital_action_data(input_handle, self.handle)
  }
}

pub trait InputActionData {
  fn is_active(&self) -> bool;
  fn to_string(&self) -> String;
}

impl InputActionData for InputAnalogActionData_t {
  fn is_active(&self) -> bool {
    self.bActive
  }
  fn to_string(&self) -> String {
    // since InputAnalogActionData_t is #[repr(packed)]
    // we can't reference its fields directly
    // so we have to copy them to local variables
    let e_mode = self.eMode;
    let x = self.x;
    let y = self.y;
    format!("Analog {{ mode: {:?}, x: {}, y: {} }}", e_mode, x, y)
  }
}

impl InputActionData for InputDigitalActionData_t {
  fn is_active(&self) -> bool {
    self.bActive
  }
  fn to_string(&self) -> String {
    format!("Digital {{ state: {} }}", self.bState)
  }
}
