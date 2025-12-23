use vigem_client::{Client, TargetId, XGamepad, Xbox360Wired};

pub struct GamePadController {
  xbox: Xbox360Wired<Client>,
}

impl GamePadController {
  pub fn new() -> Self {
    let mut xbox = Xbox360Wired::new(
      Client::connect().expect("Failed to connect to the ViGEmBus driver"),
      TargetId::XBOX360_WIRED,
    );

    xbox
      .plugin()
      .expect("Failed to plugin the virtual controller");

    xbox
      .wait_ready()
      .expect("Failed to wait for the virtual controller to be ready");

    Self { xbox }
  }

  /// Apply the gamepad state.
  pub fn apply(&mut self, data: &XGamepad) {
    self
      .xbox
      .update(data)
      .expect("Failed to update the virtual controller")
  }
}
