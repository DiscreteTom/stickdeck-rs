mod config;
mod error;
mod gamepad;
mod input;
mod server;
mod utils;

use clap::Parser;
use config::Config;
use iced::{
  alignment::Horizontal,
  executor, time,
  widget::{button, column, slider, text, toggler},
  window, Application, Command, Element, Length, Settings, Theme,
};
use input::InputConfig;
use local_ip_address::local_ip;
use std::{env, net::IpAddr, sync::mpsc};
use stickdeck_common::perf;
use tokio::sync::watch;

/// Turn your Steam Deck into a joystick for your PC, with trackpad and gyro support!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Port to bind the server to
  #[arg(short, long, default_value = "7777")]
  port: u16,
}

fn main() {
  let args = Args::parse();
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info")
  }
  env_logger::init();

  let (input_config_tx, input_config_rx) = mpsc::channel();
  input::spawn(input_config_rx).expect("Failed to spawn the input thread");

  App::run(Settings::with_flags(Flags {
    input_config_tx,
    config: Config::init(),
    port: args.port,
  }))
  .expect("Failed to run the app");
}

struct Flags {
  input_config_tx: mpsc::Sender<InputConfig>,
  config: Config,
  port: u16,
}

enum State {
  Home,
  Started,
}

#[derive(Debug, Clone)]
enum Message {
  SetDarkMode(bool),
  SetDebugMode(bool),
  SetInputUpdateInterval(u64),
  StartServer,
  Update,
  Exit,
}

struct App {
  flags: Flags,
  local_ip: IpAddr,
  port: u16,
  state: State,
  content: String,
  ui_tx: watch::Sender<String>,
  ui_rx: watch::Receiver<String>,
  ui_update_interval_ms: u64,
  debug: bool,
}

impl Application for App {
  type Executor = executor::Default;
  type Flags = Flags;
  type Message = Message;
  type Theme = Theme;

  fn new(flags: Self::Flags) -> (App, Command<Self::Message>) {
    let (ui_tx, ui_rx) = watch::channel("".to_string());
    (
      App {
        local_ip: local_ip().expect("Failed to get local ip address"),
        port: flags.port,
        state: State::Home,
        content: "".into(),
        ui_tx,
        ui_rx,
        flags,
        debug: false,
        ui_update_interval_ms: 30,
      },
      window::maximize(true),
    )
  }

  fn title(&self) -> String {
    "StickDeck".into()
  }

  fn theme(&self) -> Self::Theme {
    if self.flags.config.dark {
      Theme::Dark
    } else {
      Theme::Light
    }
  }

  fn view(&self) -> Element<Message> {
    match self.state {
      State::Home => column![
        button(
          text("Exit")
            .size(30)
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::Exit)
        .width(Length::Fill),
        column![
          text(format!(
            "Input Update Interval: {}ms",
            self.flags.config.input_update_interval_ms
          ))
          .size(20),
          slider(
            1.0..=50.0,
            self.flags.config.input_update_interval_ms as f64,
            |v| { Message::SetInputUpdateInterval(v as u64) }
          )
          .height(40)
          .step(1.0),
          toggler(Some("Dark Mode".into()), self.flags.config.dark, |v| {
            Message::SetDarkMode(v)
          })
          .size(40)
          .text_size(40)
        ]
        .padding([16, 0]),
        button(
          text("Start Server")
            .size(30)
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::StartServer)
        .width(Length::Fill),
        text(format!("stickdeck v{}", clap::crate_version!())).size(20)
      ]
      .padding([40, 80])
      .into(),
      State::Started => column![
        button(
          text("Exit")
            .size(30)
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::Exit)
        .width(Length::Fill),
        toggler(
          Some("Show Debug Info (will leak memory)".into()),
          self.debug,
          |v| { Message::SetDebugMode(v) }
        )
        .size(40)
        .text_size(40),
        text(format!(
          "=== [stickdeck v{}] Server is listening at {}:{} ===",
          clap::crate_version!(),
          self.local_ip,
          self.port
        ))
        .size(20),
        // TODO: show content will cause memory leak, fix it
        text(if self.debug { &self.content } else { "" }).size(16)
      ]
      .padding([40, 80])
      .into(),
    }
  }

  fn subscription(&self) -> iced::Subscription<Self::Message> {
    time::every(time::Duration::from_millis(self.ui_update_interval_ms)).map(|_| Message::Update)
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      Message::SetDarkMode(dark) => {
        self.flags.config.dark = dark;
        self.flags.config.save();
      }
      Message::SetDebugMode(debug) => {
        self.debug = debug;
      }
      Message::SetInputUpdateInterval(interval) => {
        self.flags.config.input_update_interval_ms = interval;
        self.flags.config.save();
      }
      Message::StartServer => {
        let (connected_tx, connected_rx) = mpsc::channel();

        server::spawn(&format!("{}:{}", self.local_ip, self.port), connected_tx);

        self
          .flags
          .input_config_tx
          .send(InputConfig {
            interval_ms: self.flags.config.input_update_interval_ms,
            ui_tx: self.ui_tx.clone(),
            connected_rx,
            ui_update_interval_ms: self.ui_update_interval_ms as u128,
          })
          .expect("Failed to send config to the input thread");

        self.state = State::Started;
      }
      Message::Update => {
        self.content = perf!("ui update", self.ui_rx.borrow().clone(), 100);
      }
      Message::Exit => {
        std::process::exit(0);
      }
    }

    Command::none()
  }
}
