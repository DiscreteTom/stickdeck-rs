use crate::all_deck_ctrl::AllDeckControls;
use std::{
  sync::{mpsc, Arc, Mutex},
  thread,
  time::Duration,
};
use steamworks::{Client, ClientManager, Input, SResult, SingleClient};
use steamworks_sys::InputHandle_t;

/// Run a function until it returns a value.
/// If the function returns [`None`], wait for the specified interval and run the Steam callbacks.
fn poll<R>(single: &SingleClient, interval_ms: u64, mut f: impl FnMut() -> Option<R>) -> R {
  loop {
    // call the function immediately, in case it can return a value without waiting
    if let Some(r) = f() {
      return r;
    }

    thread::sleep(Duration::from_millis(interval_ms));
    single.run_callbacks();
  }
}

/// Poll to get connected controller handles.
fn poll_input_handles(
  single: &SingleClient,
  input: &Input<ClientManager>,
  interval_ms: u64,
) -> Vec<InputHandle_t> {
  poll(&single, interval_ms, || {
    let handles = input.get_connected_controllers();
    if !handles.is_empty() {
      // println!("num of input handles: {:?}", handles.len());
      return Some(handles);
    }
    // println!("no input handles, retrying...");
    return None;
  })
}

pub fn spawn(
  app_id: u32,
  interval_ms: u64,
  update_lock: Arc<Mutex<bool>>,
  tx: mpsc::Sender<String>,
) -> SResult<()> {
  let (client, single) = Client::init_app(app_id)?;

  thread::spawn(move || {
    let input = client.input();
    input.init(false);

    let mut all_deck_ctrl = poll(&single, 100, || AllDeckControls::new(&input).ok());

    let input_handles = poll_input_handles(&single, &input, 100);

    input.activate_action_set_handle(input_handles[0], all_deck_ctrl.handle);

    poll(&single, interval_ms, || {
      all_deck_ctrl.update(&input, input_handles[0]);

      let mut update = update_lock.lock().unwrap();
      if *update {
        // UI requested update
        tx.send(
          all_deck_ctrl
            .repo
            .actions
            .iter()
            .map(|a| a.borrow().to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        )
        .unwrap();
        *update = false;
      }

      None as Option<()> // run forever
    });
  });

  Ok(())
}
