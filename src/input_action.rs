use crate::utils::check_handle;
use std::marker::PhantomData;
use steamworks::{ClientManager, Input};
use steamworks_sys::{uint64, InputAnalogActionData_t, InputDigitalActionData_t, InputHandle_t};

pub struct Action<Data> {
  pub name: &'static str,
  pub handle: uint64,
  pub _phantom: PhantomData<Data>,
}

pub type AnalogAction = Action<InputAnalogActionData_t>;
pub type DigitalAction = Action<InputDigitalActionData_t>;

impl Action<InputAnalogActionData_t> {
  /// Create a new analog action.
  /// Return [`Ok`] if the handle is valid.
  pub fn new(input: &Input<ClientManager>, name: &'static str) -> Result<Self, ()> {
    Ok(Self {
      name,
      handle: check_handle(input.get_analog_action_handle(name))?,
      _phantom: PhantomData,
    })
  }

  /// Refresh the action data.
  pub fn update(
    &mut self,
    input: &Input<ClientManager>,
    input_handle: InputHandle_t,
  ) -> InputAnalogActionData_t {
    input.get_analog_action_data(input_handle, self.handle)
  }
}

impl Action<InputDigitalActionData_t> {
  /// Create a new digital action.
  /// Return [`Ok`] if the handle is valid.
  pub fn new(input: &Input<ClientManager>, name: &'static str) -> Result<Self, ()> {
    Ok(Self {
      name,
      handle: check_handle(input.get_digital_action_handle(name))?,
      _phantom: PhantomData,
    })
  }

  /// Refresh the action data.
  pub fn update(
    &mut self,
    input: &Input<ClientManager>,
    input_handle: InputHandle_t,
  ) -> InputDigitalActionData_t {
    input.get_digital_action_data(input_handle, self.handle)
  }
}
