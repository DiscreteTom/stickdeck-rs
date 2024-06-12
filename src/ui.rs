use iced::executor;
use iced::time;
use iced::widget::{button, column, text};
use iced::Element;
use iced::Length;
use iced::Settings;
use iced::{Application, Command, Theme};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
  Update,
  Exit,
}

struct Flags {
  update_interval_ms: u64,
  update_lock: Arc<Mutex<bool>>,
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
      Command::none(),
    )
  }

  fn title(&self) -> String {
    // TODO: update title by action set
    String::from("Kontroller - All Deck Controls")
  }

  fn subscription(&self) -> iced::Subscription<Self::Message> {
    time::every(Duration::from_millis(self.flags.update_interval_ms)).map(|_| Message::Update)
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      Message::Update => {
        self.set_update();
        self.content = self.flags.rx.recv().unwrap();
      }
      Message::Exit => {
        std::process::exit(0);
      }
    }

    Command::none()
  }

  fn view(&self) -> Element<Message> {
    column![
      button(text("Exit").size(5))
        .on_press(Message::Exit)
        .width(Length::Fill),
      text(&self.content).size(5)
    ]
    .into()
  }
}

impl App {
  fn set_update(&mut self) {
    let mut update_lock = self.flags.update_lock.lock().unwrap();

    // set to true if not already set
    if !*update_lock {
      *update_lock = true;
    }
  }
}

/// Run the UI, block until the window is closed.
pub fn run(
  update_interval_ms: u64,
  update_lock: Arc<Mutex<bool>>,
  rx: mpsc::Receiver<String>,
) -> iced::Result {
  App::run(Settings::with_flags(Flags {
    update_interval_ms,
    update_lock,
    rx,
  }))
}
