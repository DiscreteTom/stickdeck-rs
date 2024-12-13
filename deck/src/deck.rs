mod config;
mod gamepad;
mod input;
mod server;
mod utils;

use config::Config;
use iced::{
  alignment::Horizontal,
  time,
  widget::{button, column, slider, text, toggler},
  Element, Length, Theme,
};
use input::InputConfig;
use local_ip_address::local_ip;
use std::{env, net::IpAddr, sync::mpsc};
use stickdeck_common::perf;
use tokio::sync::watch;

fn main() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info")
  }
  env_logger::init();

  iced::application("StickDeck", App::update, App::view)
    .theme(App::theme)
    .subscription(App::subscription)
    .run()
    .unwrap();
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
  input_config_tx: mpsc::Sender<InputConfig>,
  config: Config,
  local_ip: IpAddr,
  port: u16,
  state: State,
  content: String,
  ui_tx: watch::Sender<String>,
  ui_rx: watch::Receiver<String>,
  ui_update_interval_ms: u64,
  debug: bool,
}

impl Default for App {
  fn default() -> Self {
    let (input_config_tx, input_config_rx) = mpsc::channel();
    input::spawn(input_config_rx).expect("Failed to spawn the input thread");

    let (ui_tx, ui_rx) = watch::channel("".to_string());

    App {
      local_ip: local_ip().expect("Failed to get local ip address"),
      port: 7777,
      state: State::Home,
      content: "".into(),
      ui_tx,
      ui_rx,
      input_config_tx,
      config: Config::init(),
      debug: false,
      ui_update_interval_ms: 30,
    }
  }
}

impl App {
  fn theme(&self) -> Theme {
    if self.config.dark {
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
            .align_x(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::Exit)
        .width(Length::Fill),
        column![
          text(format!(
            "Input Update Interval: {}ms",
            self.config.input_update_interval_ms
          ))
          .size(20),
          slider(
            1.0..=50.0,
            self.config.input_update_interval_ms as f64,
            |v| { Message::SetInputUpdateInterval(v as u64) }
          )
          .height(40)
          .step(1.0),
          toggler(self.config.dark,)
            .label("Dark Mode")
            .on_toggle(|v| { Message::SetDarkMode(v) })
            .size(40)
            .text_size(40)
        ]
        .padding([16, 0]),
        button(
          text("Start Server")
            .size(30)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::StartServer)
        .width(Length::Fill),
        text(format!("stickdeck v{}", env!("CARGO_PKG_VERSION"))).size(20)
      ]
      .padding([40, 80])
      .into(),
      State::Started => column![
        button(
          text("Exit")
            .size(30)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::Exit)
        .width(Length::Fill),
        toggler(self.debug,)
          .label("Show Debug Info (will leak memory)")
          .on_toggle(|v| { Message::SetDebugMode(v) })
          .size(40)
          .text_size(40),
        text(format!(
          "=== [stickdeck v{}] Server is listening at {}:{} ===",
          env!("CARGO_PKG_VERSION"),
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

  fn subscription(&self) -> iced::Subscription<Message> {
    time::every(time::Duration::from_millis(self.ui_update_interval_ms)).map(|_| Message::Update)
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::SetDarkMode(dark) => {
        self.config.dark = dark;
        self.config.save();
      }
      Message::SetDebugMode(debug) => {
        self.debug = debug;
      }
      Message::SetInputUpdateInterval(interval) => {
        self.config.input_update_interval_ms = interval;
        self.config.save();
      }
      Message::StartServer => {
        let (connected_tx, connected_rx) = mpsc::channel();

        server::spawn(&format!("{}:{}", self.local_ip, self.port), connected_tx);

        self
          .input_config_tx
          .send(InputConfig {
            interval_ms: self.config.input_update_interval_ms,
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
  }
}
