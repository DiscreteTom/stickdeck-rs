mod action;
mod xbox;

use crate::gamepad::{XButtons, XGamepad};
use action::{InputAction, InputActionData, InputDigitalAction, UpdatableInputAction};
use log::info;
use std::{sync::mpsc, thread, time::Duration};
use steamworks::{Client, ClientManager, Input, SResult, SingleClient};
use steamworks_sys::InputHandle_t;
use xbox::XBoxControls;

pub struct InputConfig {
  pub interval_ms: u64,
  pub update_rx: mpsc::Receiver<()>,
  pub ui_tx: mpsc::Sender<String>,
  pub connected_rx: mpsc::Receiver<mpsc::Sender<XGamepad>>,
}

pub fn spawn(input_rx: mpsc::Receiver<InputConfig>) -> SResult<()> {
  let (client, single) = Client::init()?;

  thread::spawn(move || {
    let input = client.input();
    input.init(false);

    // try to init controls from vdf
    let xbox = poll(&single, 100, retry(10, || XBoxControls::new(&input).ok()));
    info!("XBox controls initialized");

    // try to get input handles (input devices)
    let input_handles = poll(
      &single,
      100,
      retry(10, || {
        let handles = input.get_connected_controllers();
        if !handles.is_empty() {
          info!("num of input handles: {:?}", handles.len());
          return Some(handles);
        }
        info!("no input handles, retrying...");
        return None;
      }),
    );

    // enable xbox control action set for the first input handle
    input.activate_action_set_handle(input_handles[0], xbox.handle);

    let InputConfig {
      interval_ms,
      update_rx,
      ui_tx,
      connected_rx,
    } = input_rx.recv().expect("Failed to receive input data");
    let mut net_tx = None;
    let mut last_gamepad = XGamepad::default();

    poll(
      &single,
      interval_ms,
      forever(|| {
        // check if the client is connected
        if net_tx.is_none() {
          net_tx = connected_rx.try_recv().ok();
        }

        // prepare ctx
        let mut ui_str = update_rx.try_recv().ok().map(|_| String::new());
        let mut ctx = (&input, input_handles[0], &mut ui_str);

        let mut gamepad = XGamepad::default();

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
          handle_mouse(data.x, false, |x| gamepad.thumb_lx = x);
          handle_mouse(data.y, true, |y| gamepad.thumb_ly = y);
        });
        update_input(&xbox.r_move, &mut ctx, |data| {
          gamepad.thumb_rx = f32_to_i16(data.x);
          gamepad.thumb_ry = f32_to_i16(data.y);
        });
        update_input(&xbox.r_mouse, &mut ctx, |data| {
          handle_mouse(data.x, false, |x| gamepad.thumb_rx = x);
          handle_mouse(data.y, true, |y| gamepad.thumb_ry = y);
        });

        // info!("{:?}", std::time::SystemTime::now());
        // info!("{:?}", gamepad);

        // only send data if client is connected
        net_tx.as_ref().map(|tx| {
          // only send data if it's changed
          if !gamepad_eq(&gamepad, &last_gamepad) {
            tx.send(gamepad.clone())
              .expect("Failed to send gamepad data");
            last_gamepad = gamepad;
          }
        });
        ui_str.map(|s| ui_tx.send(s).expect("Failed to send UI data"));
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

/// Make `f` retry-able for `n` times before panicking when [`poll`]ed.
fn retry<R>(mut n: usize, mut f: impl FnMut() -> Option<R>) -> impl FnMut() -> Option<R> {
  move || {
    if n == 0 {
      panic!("Retry limit exceeded");
    }
    n -= 1;
    f()
  }
}

/// Wrap a function to run forever when [`poll`]ed.
fn forever(mut f: impl FnMut()) -> impl FnMut() -> Option<()> {
  move || {
    f();
    None
  }
}

fn update_input<Data: InputActionData>(
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
      .map(|s| s.push_str(&format!("{}: {}\n", action.name, data.to_string())));
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

fn handle_mouse(n: f32, reverse: bool, mut cb: impl FnMut(i16)) {
  if n == 0.0 {
    // if 0, don't override the stick value
    return;
  }

  // TODO: make this configurable?
  // actually user can directly modify the gyro/trackpad sensitivity in Steam
  // TODO: add some debounce logic?
  let sensitivity = 5000.0;

  // crop the result to [-32767, 32767]
  let mapped = (n * sensitivity).max(-32767.0).min(32767.0) as i16;

  cb(if reverse { -mapped } else { mapped });
}

fn gamepad_eq(a: &XGamepad, b: &XGamepad) -> bool {
  a.buttons.raw == b.buttons.raw
    && a.left_trigger == b.left_trigger
    && a.right_trigger == b.right_trigger
    && a.thumb_lx == b.thumb_lx
    && a.thumb_ly == b.thumb_ly
    && a.thumb_rx == b.thumb_rx
    && a.thumb_ry == b.thumb_ry
}
