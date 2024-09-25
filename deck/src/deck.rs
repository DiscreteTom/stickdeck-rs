mod config;
mod gamepad;
mod input;
mod perf;
mod server;
mod utils;

use config::Config;
use iced::{
  alignment::Horizontal,
  executor,
  subscription::unfold,
  widget::{button, column, slider, text, toggler},
  window, Application, Command, Element, Length, Settings, Theme,
};
use input::InputConfig;
use local_ip_address::local_ip;
use perf::perf;
use std::{env, net::IpAddr, sync::mpsc};
use tokio::sync::watch;

fn main() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info")
  }
  env_logger::init();

  let (input_config_tx, input_config_rx) = mpsc::channel();
  input::spawn(input_config_rx).expect("Failed to spawn the input thread");

  App::run(Settings::with_flags(Flags {
    input_config_tx,
    config: Config::init(),
  }))
  .expect("Failed to run the app");
}

struct Flags {
  input_config_tx: mpsc::Sender<InputConfig>,
  config: Config,
}

enum State {
  Home,
  Started,
}

#[derive(Debug, Clone)]
enum Message {
  SetDarkMode(bool),
  SetInputUpdateInterval(u64),
  StartServer,
  Update(String),
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
        port: 7777,
        state: State::Home,
        content: "".into(),
        ui_tx,
        ui_rx,
        flags,
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
        text(format!("stickdeck v{}", env!("CARGO_PKG_VERSION"))).size(20)
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
        text(format!(
          "=== [stickdeck v{}] Server is listening at {}:{} ===",
          env!("CARGO_PKG_VERSION"),
          self.local_ip,
          self.port
        ))
        .size(20),
        text(&self.content).size(16)
      ]
      .padding([40, 80])
      .into(),
    }
  }

  fn subscription(&self) -> iced::Subscription<Self::Message> {
    unfold("ui update", self.ui_rx.clone(), move |mut rx| async move {
      perf!(
        "ui update",
        rx.changed().await.expect("ui content channel closed"),
        100
      );
      let content = rx.borrow_and_update().clone();
      (Message::Update(content), rx)
    })
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      Message::SetDarkMode(dark) => {
        self.flags.config.dark = dark;
        self.flags.config.save();
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
            ui_update_interval_ms: 30,
          })
          .expect("Failed to send config to the input thread");

        self.state = State::Started;
      }
      Message::Update(content) => {
        self.content = content;
      }
      Message::Exit => {
        std::process::exit(0);
      }
    }

    Command::none()
  }
}
