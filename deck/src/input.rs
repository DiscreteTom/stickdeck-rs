mod action;
mod xbox;

use crate::gamepad::{XButtons, XGamepad};
use action::{InputAction, InputActionData, InputDigitalAction, UpdatableInputAction};
use std::{
  fmt::Debug,
  sync::{mpsc, Arc, Mutex},
  thread,
  time::Duration,
};
use steamworks::{Client, ClientManager, Input, SResult, SingleClient};
use steamworks_sys::InputHandle_t;
use xbox::XBoxControls;

pub fn spawn(
  app_id: u32,
  interval_ms: u64,
  update_lock: Arc<Mutex<bool>>,
  tx: mpsc::Sender<String>,
  net_tx: mpsc::Sender<XGamepad>,
) -> SResult<()> {
  let (client, single) = Client::init_app(app_id)?;

  thread::spawn(move || {
    let input = client.input();
    input.init(false);

    // try to init controls from vdf
    let xbox = poll(&single, 100, retry(10, || XBoxControls::new(&input).ok()));
    println!("XBox controls initialized");

    // try to get input handles (input devices)
    let input_handles = poll(
      &single,
      100,
      retry(10, || {
        let handles = input.get_connected_controllers();
        if !handles.is_empty() {
          println!("num of input handles: {:?}", handles.len());
          return Some(handles);
        }
        println!("no input handles, retrying...");
        return None;
      }),
    );

    // enable xbox control action set for the first input handle
    input.activate_action_set_handle(input_handles[0], xbox.handle);

    poll(
      &single,
      interval_ms,
      forever(|| {
        let mut gamepad = XGamepad::default();
        let mut update_ui = update_lock.lock().unwrap();
        let mut ui_str = update_ui.then(|| String::new());

        let mut ctx = (&input, input_handles[0], &mut ui_str);
        input.run_frame();

        // digital buttons
        let raw = &mut gamepad.buttons.raw;
        update_btn(&xbox.btn_up, &mut ctx, || *raw |= XButtons::UP);
        update_btn(&xbox.btn_down, &mut ctx, || *raw |= XButtons::DOWN);
        update_btn(&xbox.btn_left, &mut ctx, || *raw |= XButtons::LEFT);
        update_btn(&xbox.btn_right, &mut ctx, || *raw |= XButtons::RIGHT);
        update_btn(&xbox.btn_start, &mut ctx, || *raw |= XButtons::START);
        update_btn(&xbox.btn_back, &mut ctx, || *raw |= XButtons::BACK);
        update_btn(&xbox.btn_l_thumb, &mut ctx, || *raw |= XButtons::LTHUMB);
        update_btn(&xbox.btn_r_thumb, &mut ctx, || *raw |= XButtons::RTHUMB);
        update_btn(&xbox.btn_lb, &mut ctx, || *raw |= XButtons::LB);
        update_btn(&xbox.btn_rb, &mut ctx, || *raw |= XButtons::RB);
        update_btn(&xbox.btn_a, &mut ctx, || *raw |= XButtons::A);
        update_btn(&xbox.btn_b, &mut ctx, || *raw |= XButtons::B);
        update_btn(&xbox.btn_x, &mut ctx, || *raw |= XButtons::X);
        update_btn(&xbox.btn_y, &mut ctx, || *raw |= XButtons::Y);

        // analog actions
        update_input(&xbox.lt, &mut ctx, |data| {
          gamepad.left_trigger = f32_to_u8(data.x)
        });
        update_input(&xbox.rt, &mut ctx, |data| {
          gamepad.right_trigger = f32_to_u8(data.x)
        });
        update_input(&xbox.l_move, &mut ctx, |data| {
          gamepad.thumb_lx = f32_to_i16(data.x);
          gamepad.thumb_ly = f32_to_i16(data.y);
        });
        update_input(&xbox.l_mouse, &mut ctx, |data| {
          // only update if stick is not at 0
          if_stick_not_0(data.x, |x| gamepad.thumb_lx = x);
          if_stick_not_0(data.y, |y| gamepad.thumb_ly = y);
        });
        update_input(&xbox.r_move, &mut ctx, |data| {
          gamepad.thumb_rx = f32_to_i16(data.x);
          gamepad.thumb_ry = f32_to_i16(data.y);
        });
        update_input(&xbox.r_mouse, &mut ctx, |data| {
          // only update if stick is not at 0
          if_stick_not_0(data.x, |x| gamepad.thumb_rx = x);
          if_stick_not_0(data.y, |y| gamepad.thumb_ry = y);
        });

        // println!("{:?}", std::time::SystemTime::now());
        // println!("{:?}", gamepad);
        net_tx.send(gamepad).expect("Failed to send gamepad data");

        ui_str.map(|s| {
          // UI requested update
          tx.send(s).expect("Failed to send UI data");
          *update_ui = false;
        });
      }),
    );
  });

  Ok(())
}

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

/// Make `f` retry-able for `n` times before panicking.
fn retry<R>(mut n: usize, mut f: impl FnMut() -> Option<R>) -> impl FnMut() -> Option<R> {
  move || {
    if n == 0 {
      panic!("Retry limit exceeded");
    }
    n -= 1;
    f()
  }
}

/// Wrap a function to run forever when polled.
fn forever(mut f: impl FnMut()) -> impl FnMut() -> Option<()> {
  move || {
    f();
    None
  }
}

fn update_input<Data: InputActionData + Debug>(
  action: &InputAction<Data>,
  (input, input_handle, ui_str): &mut (&Input<ClientManager>, InputHandle_t, &mut Option<String>),
  mut cb: impl FnMut(&Data),
) where
  InputAction<Data>: UpdatableInputAction<Data>,
{
  let data = action.update(input, *input_handle);
  if data.is_active() {
    ui_str
      .as_mut()
      .map(|s| s.push_str(&format!("{}: {:?}\n", action.name, data)));
    cb(&data);
  }
}

fn update_btn(
  action: &InputDigitalAction,
  ctx: &mut (&Input<ClientManager>, InputHandle_t, &mut Option<String>),
  mut cb: impl FnMut(),
) {
  update_input(action, ctx, |data| {
    if data.bState {
      cb()
    }
  });
}

/// Convert f32 `[0, 1]` to u8 `[0, 255]`
fn f32_to_u8(f: f32) -> u8 {
  (f * 255.0) as u8
}

/// Convert f32 `(-1, 1)` to i16 `[-32768, 32767]`
fn f32_to_i16(f: f32) -> i16 {
  (f * 32767.0) as i16
}

fn if_stick_not_0(n: f32, mut cb: impl FnMut(i16)) {
  if n != 0.0 {
    cb(f32_to_i16(n));
  }
}
