mod client;
mod controller;

use clap::Parser;
use log::{debug, info, log_enabled, trace, Level};
use std::{env, sync::mpsc, time::Instant};
use stickdeck_common::{perf, Mouse, MouseButton, Packet};
use windows::Win32::UI::Input::KeyboardAndMouse::{
  SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
  MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEINPUT,
  MOUSE_EVENT_FLAGS,
};

use crate::controller::Controller;

/// Turn your Steam Deck into a joystick for your PC, with trackpad and gyro support!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Server address to connect to
  #[arg(default_value = "steamdeck")]
  server: String,

  /// Server port to connect to
  #[arg(short, long, default_value = "7777")]
  port: u16,
}

fn main() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info")
  }
  env_logger::init();

  let args = Args::parse();

  info!("stickdeck-win v{}", clap::crate_version!());
  info!("See https://github.com/DiscreteTom/stickdeck-rs for more info.");

  let (gamepad_tx, gamepad_rx) = mpsc::sync_channel(8);

  // connect to the server
  client::spawn(&format!("{}:{}", args.server, args.port), gamepad_tx);

  let mut controller = Controller::new();
  info!("Virtual controller is ready");

  let mut move_mouse = init_mouse();

  let mut now = Instant::now();
  let mut count = 0;
  while let Ok(data) = gamepad_rx.recv() {
    trace!("Got {:?}", data);

    match data {
      Packet::Timestamp(_timestamp) => {} // TODO
      Packet::Gamepad(gamepad) => perf!("update controller", controller.apply(&gamepad), 10),
      Packet::Mouse(data) => perf!("move mouse", move_mouse(&data), 10),
    }

    if log_enabled!(Level::Debug) {
      count += 1;
      if now.elapsed().as_secs() >= 1 {
        debug!("{} updates per second", count);
        now = Instant::now();
        count = 0;
      }
    }
  }

  info!("Shutting down...");
}

fn init_mouse() -> impl FnMut(&Mouse) {
  let mut input = INPUT {
    r#type: INPUT_MOUSE,
    Anonymous: INPUT_0 {
      mi: MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSE_EVENT_FLAGS(0),
        time: 0,
        dwExtraInfo: 0,
      },
    },
  };
  let size = std::mem::size_of_val(&input) as i32;
  let mut last_mb = MouseButton::default();

  move |data: &Mouse| unsafe {
    input.Anonymous.mi.dx = data.x as i32;
    input.Anonymous.mi.dy = data.y as i32;
    input.Anonymous.mi.dwFlags.0 = 0;
    input.Anonymous.mi.mouseData = data.scroll as u32;
    if data.x != 0 || data.y != 0 {
      input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_MOVE.0;
    }
    if data.buttons != last_mb {
      if data.buttons.is_left_button_down() != last_mb.is_left_button_down() {
        if data.buttons.is_left_button_down() {
          input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_LEFTDOWN.0;
        } else {
          input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_LEFTUP.0;
        }
      }
      if data.buttons.is_right_button_down() != last_mb.is_right_button_down() {
        if data.buttons.is_right_button_down() {
          input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_RIGHTDOWN.0;
        } else {
          input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_RIGHTUP.0;
        }
      }
      last_mb = data.buttons;
    }
    if data.scroll != 0 {
      input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_WHEEL.0;
    }
    SendInput(&[input], size);
  }
}
