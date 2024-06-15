mod gamepad;
mod input;
mod server;
mod utils;

use iced::{
  alignment::Horizontal,
  executor, time,
  widget::{button, column, row, slider, text},
  window, Application, Command, Element, Length, Settings, Theme,
};
use input::InputConfig;
use local_ip_address::local_ip;
use std::{net::IpAddr, sync::mpsc, time::Duration};

fn main() {
  let (input_config_tx, input_config_rx) = mpsc::channel();
  input::spawn(
    480, // TODO: replace 480 with the real AppID
    input_config_rx,
  )
  .expect("Failed to spawn the input thread");

  App::run(Settings::with_flags(Flags { input_config_tx })).expect("Failed to run the app");
}

struct Flags {
  input_config_tx: mpsc::Sender<InputConfig>,
}

enum State {
  Home,
  Started {
    update_tx: mpsc::Sender<()>,
    ui_rx: mpsc::Receiver<String>,
  },
}

#[derive(Debug, Clone)]
enum Message {
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
  ui_update_interval_ms: u64,
  input_update_interval_ms: u64,
}

impl Application for App {
  type Executor = executor::Default;
  type Flags = Flags;
  type Message = Message;
  type Theme = Theme;

  fn new(flags: Self::Flags) -> (App, Command<Self::Message>) {
    (
      App {
        flags,
        local_ip: local_ip().expect("Failed to get local ip address"),
        port: 7777,
        state: State::Home,
        content: "".into(),
        ui_update_interval_ms: 30,
        input_update_interval_ms: 10,
      },
      window::maximize(true),
    )
  }

  fn title(&self) -> String {
    "Stickdeck".into()
  }

  fn view(&self) -> Element<Message> {
    match self.state {
      State::Home => column![
        button(
          text("Exit")
            .size(5)
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::Exit)
        .width(Length::Fill),
        row![
          text("Input Update Interval (ms)").size(5),
          slider(1.0..=50.0, self.input_update_interval_ms as f64, |v| {
            Message::SetInputUpdateInterval(v as u64)
          })
          .step(1.0),
        ],
        button(
          text("Start Server")
            .size(5)
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::StartServer)
        .width(Length::Fill),
      ]
      .into(),
      State::Started { .. } => column![
        button(
          text("Exit")
            .size(5)
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::Exit)
        .width(Length::Fill),
        text(&self.content).size(5)
      ]
      .into(),
    }
  }

  fn subscription(&self) -> iced::Subscription<Self::Message> {
    time::every(Duration::from_millis(self.ui_update_interval_ms)).map(|_| Message::Update)
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      Message::SetInputUpdateInterval(interval) => {
        self.input_update_interval_ms = interval;
      }
      Message::StartServer => {
        let (update_tx, update_rx) = mpsc::channel();
        let (ui_tx, ui_rx) = mpsc::channel();
        let (net_tx, net_rx) = mpsc::channel();

        server::spawn(&format!("{}:{}", self.local_ip, self.port), net_rx);

        self
          .flags
          .input_config_tx
          .send(InputConfig {
            interval_ms: self.input_update_interval_ms,
            update_rx,
            ui_tx,
            net_tx,
          })
          .expect("Failed to send config to the input thread");

        self.state = State::Started { update_tx, ui_rx };
      }
      Message::Update => {
        if let State::Started { update_tx, ui_rx } = &self.state {
          update_tx.send(()).expect("Failed to send update signal");
          ui_rx
            .recv()
            .map(|content| {
              self.content = format!(
                "Server is listening at {}:{}\n{}",
                self.local_ip, self.port, content
              )
            })
            .expect("Failed to receive data from the input thread");
        }
      }
      Message::Exit => {
        std::process::exit(0);
      }
    }

    Command::none()
  }
}
