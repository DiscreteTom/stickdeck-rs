use std::{cell::RefCell, rc::Rc};
use steamworks::{ClientManager, Input};
use steamworks_sys::{
  InputAnalogActionData_t, InputDigitalActionData_t, InputDigitalActionHandle_t, InputHandle_t,
};

pub trait Action {
  /// Refresh the action data.
  fn update(&mut self, input: &Input<ClientManager>, input_handle: InputHandle_t);
  /// Return a string representation of the action data.
  fn to_string(&self) -> String;
}

// TODO: add `enabled`?
#[derive(Clone, Debug)]
pub struct AnalogAction {
  pub name: String,
  pub handle: InputDigitalActionHandle_t,
  pub data: Option<InputAnalogActionData_t>,
}

#[derive(Clone, Debug)]
pub struct DigitalAction {
  pub name: String,
  pub handle: InputDigitalActionHandle_t,
  pub data: Option<InputDigitalActionData_t>,
}

impl AnalogAction {
  /// Create a new analog action.
  /// Return `Ok` if the handle is valid.
  pub fn new(input: &Input<ClientManager>, action_name: &str) -> Result<Self, ()> {
    Ok(Self {
      name: String::from(action_name),
      handle: check_handle(input.get_analog_action_handle(action_name))?,
      data: None,
    })
  }
}

/// Create a new digital action.
/// Return `Ok` if the handle is valid.
impl DigitalAction {
  pub fn new(input: &Input<ClientManager>, action_name: &str) -> Result<Self, ()> {
    Ok(Self {
      name: String::from(action_name),
      handle: check_handle(input.get_digital_action_handle(action_name))?,
      data: None,
    })
  }
}

impl Action for AnalogAction {
  fn update(&mut self, input: &Input<ClientManager>, input_handle: InputHandle_t) {
    self.data = Some(input.get_analog_action_data(input_handle, self.handle));
  }

  fn to_string(&self) -> String {
    format!(
      "{}: {:?}",
      self.name,
      match self.data {
        Some(data) => data,
        None => return String::from("None"),
      }
    )
  }
}

impl Action for DigitalAction {
  fn update(&mut self, input: &Input<ClientManager>, input_handle: InputHandle_t) {
    self.data = Some(input.get_digital_action_data(input_handle, self.handle));
  }

  fn to_string(&self) -> String {
    format!(
      "{}: {:?}",
      self.name,
      match self.data {
        Some(data) => data,
        None => return String::from("None"),
      }
    )
  }
}

pub struct ActionRepo {
  pub actions: Vec<Rc<RefCell<dyn Action>>>,
}

impl ActionRepo {
  pub fn new() -> Self {
    Self {
      actions: Vec::new(),
    }
  }

  /// Append the action to the repo and return the action.
  pub fn append<T>(&mut self, action: T) -> Rc<RefCell<T>>
  where
    T: Action + 'static,
  {
    let rc = Rc::new(RefCell::new(action));
    self.actions.push(rc.clone());
    rc
  }

  pub fn update(&mut self, input: &Input<ClientManager>, input_handle: InputHandle_t) {
    for action in &mut self.actions {
      action.borrow_mut().update(input, input_handle);
    }
  }
}

pub type WrappedDigitalAction = Rc<RefCell<DigitalAction>>;
pub type WrappedAnalogAction = Rc<RefCell<AnalogAction>>;

/// Return `Err` if the handle's value is 0.
pub fn check_handle(handle: u64) -> Result<u64, ()> {
  if handle == 0 {
    Err(())
  } else {
    Ok(handle)
  }
}
