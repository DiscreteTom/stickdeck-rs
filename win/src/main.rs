mod client;
mod controller;
mod mouse;

use clap::Parser;
use log::{debug, info, log_enabled, trace, Level};
use std::{env, sync::mpsc, time::Instant};
use stickdeck_common::{perf, Packet};

use crate::{controller::Controller, mouse::MouseController};

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

  let mut mouse = MouseController::new();

  let mut now = Instant::now();
  let mut count = 0;
  while let Ok(data) = gamepad_rx.recv() {
    trace!("Got {:?}", data);

    match data {
      Packet::Timestamp(_timestamp) => {} // TODO
      Packet::Gamepad(gamepad) => perf!("update controller", controller.apply(&gamepad), 10),
      Packet::Mouse(data) => perf!("move mouse", mouse.apply(&data), 10),
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
