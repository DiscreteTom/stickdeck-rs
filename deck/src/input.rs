mod action;
mod xbox;

use crate::gamepad::{XButtons, XGamepad};
use action::{InputAction, InputActionData, InputDigitalAction, UpdatableInputAction};
use log::{info, trace};
use std::{sync::mpsc, thread, time::Duration};
use steamworks::{Client, ClientManager, Input, SResult, SingleClient};
use steamworks_sys::InputHandle_t;
use stickdeck_common::{MouseMove, Packet};
use xbox::XBoxControls;

pub struct InputConfig {
  pub interval_ms: u64,
  pub update_rx: mpsc::Receiver<()>,
  pub ui_tx: mpsc::Sender<String>,
  pub connected_rx: mpsc::Receiver<mpsc::Sender<Packet<XGamepad>>>,
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
        let mut mouse = MouseMove::default();

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
          gamepad.left_trigger = scale_f32_to_u8(data.x)
        });
        update_input(&xbox.rt, &mut ctx, |data| {
          gamepad.right_trigger = scale_f32_to_u8(data.x)
        });
        update_input(&xbox.l_move, &mut ctx, |data| {
          gamepad.thumb_lx = scale_f32_to_i16(data.x);
          gamepad.thumb_ly = scale_f32_to_i16(data.y);
        });
        update_input(&xbox.l_mouse, &mut ctx, |data| {
          mouse.x.safe_add(crop_f32_to_i8(data.x));
          mouse.y.safe_add(crop_f32_to_i8(data.y));
        });
        update_input(&xbox.r_move, &mut ctx, |data| {
          gamepad.thumb_rx = scale_f32_to_i16(data.x);
          gamepad.thumb_ry = scale_f32_to_i16(data.y);
        });
        update_input(&xbox.r_mouse, &mut ctx, |data| {
          mouse.x.safe_add(crop_f32_to_i8(data.x));
          mouse.y.safe_add(crop_f32_to_i8(data.y));
        });

        // only send data if client is connected
        net_tx.as_ref().map(|tx| {
          // only send data if it's changed
          match (
            !gamepad_eq(&gamepad, &last_gamepad), // gamepad changed
            mouse.x != 0 || mouse.y != 0,         // mouse moved
          ) {
            (true, true) => {
              trace!("Send {:?}", (&gamepad, mouse));

              tx.send(Packet::GamepadAndMouseMove(gamepad.clone(), mouse))
                .expect("Failed to send data");
              last_gamepad = gamepad;
            }
            (true, false) => {
              trace!("Send {:?}", &gamepad);

              tx.send(Packet::Gamepad(gamepad.clone()))
                .expect("Failed to send data");
              last_gamepad = gamepad;
            }
            (false, true) => {
              trace!("Send {:?}", mouse);

              tx.send(Packet::MouseMove(mouse))
                .expect("Failed to send data");
            }
            (false, false) => (),
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

/// Convert f32 `[-128, 127]` to u8 `[-128, 127]`
fn crop_f32_to_i8(f: f32) -> i8 {
  f.max(-128.0).min(127.0) as i8
}

/// Convert f32 `[0, 1]` to u8 `[0, 255]`
fn scale_f32_to_u8(f: f32) -> u8 {
  (f * 255.0) as u8
}

/// Convert f32 `(-1, 1)` to i16 `[-32768, 32767]`
fn scale_f32_to_i16(f: f32) -> i16 {
  (f * 32767.0) as i16
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

trait SafeAdd {
  fn safe_add(&mut self, other: Self);
}

impl SafeAdd for i8 {
  fn safe_add(&mut self, other: Self) {
    *self = self.saturating_add(other);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gamepad_eq() {
    let a = XGamepad::default();
    let b = XGamepad::default();
    assert!(gamepad_eq(&a, &b));

    let mut a = XGamepad::default();
    a.buttons.raw |= XButtons::UP;
    assert!(!gamepad_eq(&a, &b));

    let mut a = XGamepad::default();
    a.left_trigger = 255;
    assert!(!gamepad_eq(&a, &b));

    let mut a = XGamepad::default();
    a.right_trigger = 255;
    assert!(!gamepad_eq(&a, &b));

    let mut a = XGamepad::default();
    a.thumb_lx = 32767;
    assert!(!gamepad_eq(&a, &b));

    let mut a = XGamepad::default();
    a.thumb_ly = 32767;
    assert!(!gamepad_eq(&a, &b));

    let mut a = XGamepad::default();
    a.thumb_rx = 32767;
    assert!(!gamepad_eq(&a, &b));

    let mut a = XGamepad::default();
    a.thumb_ry = 32767;
    assert!(!gamepad_eq(&a, &b));
  }

  #[test]
  fn test_safe_add() {
    let mut a = 127;
    a.safe_add(1);
    assert_eq!(a, 127);

    let mut a = -128;
    a.safe_add(-1);
    assert_eq!(a, -128);

    let mut a = 0;
    a.safe_add(1);
    assert_eq!(a, 1);

    let mut a = 0;
    a.safe_add(-1);
    assert_eq!(a, -1);
  }
}
