mod client;
mod gamepad;
mod input;
mod utils;

use gamepad::XGamepad;
use iced::{
  alignment::Horizontal,
  executor, time,
  widget::{button, column, text, text_input},
  window, Application, Command, Element, Length, Settings, Theme,
};
use std::{sync::mpsc, time::Duration};

fn main() {
  let (input_tx, input_rx) = mpsc::channel();
  input::spawn(
    480, // TODO: replace 480 with the real AppID
    input_rx,
  )
  .expect("Failed to spawn the input thread");
  App::run(Settings::with_flags(Flags { input_tx })).expect("Failed to run the app");
}

struct Flags {
  input_tx: mpsc::Sender<(
    u64,
    mpsc::Receiver<()>,
    mpsc::Sender<String>,
    mpsc::Sender<XGamepad>,
  )>,
}

enum State {
  Home,
  Connected {
    update_tx: mpsc::Sender<()>,
    rx: mpsc::Receiver<String>,
  },
}

#[derive(Debug, Clone)]
enum Message {
  UpdateAddr(String),
  Connect,
  Update,
  Exit,
}

struct App {
  flags: Flags,
  content: String,
  state: State,
  addr: String,
  update_interval_ms: u64,
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
        content: String::new(),
        state: State::Home,
        addr: "".into(),
        update_interval_ms: 30,
      },
      window::maximize(true),
    )
  }

  fn title(&self) -> String {
    "Stickdeck".into()
  }

  fn subscription(&self) -> iced::Subscription<Self::Message> {
    time::every(Duration::from_millis(self.update_interval_ms)).map(|_| Message::Update)
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      Message::UpdateAddr(addr) => {
        self.addr = addr;
      }
      Message::Connect => {
        let (update_tx, update_rx) = mpsc::channel();
        let (ui_tx, rx) = mpsc::channel();
        let (net_tx, net_rx) = mpsc::channel();

        client::spawn(&self.addr, net_rx);

        self
          .flags
          .input_tx
          .send((
            10, // interval of polling input events // TODO: make this configurable
            update_rx, ui_tx, net_tx,
          ))
          .expect("Failed to send config to the input thread");

        self.state = State::Connected { update_tx, rx };
      }
      Message::Update => {
        if let State::Connected { update_tx, rx } = &self.state {
          update_tx.send(()).expect("Failed to send update signal");
          rx.recv()
            .expect("Failed to receive data from the input thread");
        }
      }
      Message::Exit => {
        std::process::exit(0);
      }
    }

    Command::none()
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
        text_input("192.168.1.1:7777", &self.addr)
          .size(5)
          .on_input(|s| Message::UpdateAddr(s)),
        button(
          text("Connect")
            .size(5)
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
        )
        .on_press(Message::Connect)
        .width(Length::Fill),
      ]
      .into(),
      State::Connected { .. } => column![
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
}
