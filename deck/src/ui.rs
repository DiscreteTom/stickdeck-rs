use iced::{
  alignment::Horizontal,
  executor, time,
  widget::{button, column, text},
  window, Application, Command, Element, Length, Settings, Theme,
};
use std::{sync::mpsc, time::Duration};

#[derive(Debug, Clone)]
enum Message {
  Update,
  Exit,
}

struct Flags {
  update_interval_ms: u64,
  update_tx: mpsc::Sender<()>,
  rx: mpsc::Receiver<String>,
}

struct App {
  flags: Flags,
  content: String,
}

impl Application for App {
  type Executor = executor::Default;
  type Flags = Flags;
  type Message = Message;
  type Theme = Theme;

  fn new(flags: Flags) -> (App, Command<Self::Message>) {
    (
      App {
        flags,
        content: String::new(),
      },
      window::maximize(true),
    )
  }

  fn title(&self) -> String {
    "Stickdeck".into()
  }

  fn subscription(&self) -> iced::Subscription<Self::Message> {
    time::every(Duration::from_millis(self.flags.update_interval_ms)).map(|_| Message::Update)
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      Message::Update => {
        self
          .flags
          .update_tx
          .send(())
          .expect("Failed to send update signal");
        // wait for the input thread to update the content
        self.content = self
          .flags
          .rx
          .recv()
          .expect("Failed to receive data from the input thread");
      }
      Message::Exit => {
        std::process::exit(0);
      }
    }

    Command::none()
  }

  fn view(&self) -> Element<Message> {
    column![
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
    .into()
  }
}

/// Run the UI, block until the window is closed.
pub fn run(
  update_interval_ms: u64,
  update_tx: mpsc::Sender<()>,
  rx: mpsc::Receiver<String>,
) -> iced::Result {
  App::run(Settings::with_flags(Flags {
    update_interval_ms,
    update_tx,
    rx,
  }))
}
