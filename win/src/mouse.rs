use stickdeck_common::{Mouse, MouseButton};
use windows::Win32::UI::Input::KeyboardAndMouse::{
  SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
  MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEINPUT,
  MOUSE_EVENT_FLAGS,
};
pub struct MouseController {
  input: INPUT,
  size: i32,
  last_mb: MouseButton,
}

impl MouseController {
  pub fn new() -> Self {
    let input = INPUT {
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
    let last_mb = MouseButton::default();
    Self {
      input,
      size,
      last_mb,
    }
  }

  pub fn apply(&mut self, data: &Mouse) {
    let mut input = self.input;
    input.Anonymous.mi.dx = data.x as i32;
    input.Anonymous.mi.dy = data.y as i32;
    input.Anonymous.mi.dwFlags.0 = 0;
    input.Anonymous.mi.mouseData = data.scroll as u32;
    unsafe {
      if data.x != 0 || data.y != 0 {
        input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_MOVE.0;
      }
      if data.buttons != self.last_mb {
        if data.buttons.is_left_button_down() != self.last_mb.is_left_button_down() {
          if data.buttons.is_left_button_down() {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_LEFTDOWN.0;
          } else {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_LEFTUP.0;
          }
        }
        if data.buttons.is_right_button_down() != self.last_mb.is_right_button_down() {
          if data.buttons.is_right_button_down() {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_RIGHTDOWN.0;
          } else {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_RIGHTUP.0;
          }
        }
        self.last_mb = data.buttons;
      }
      if data.scroll != 0 {
        input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_WHEEL.0;
      }
      SendInput(&[input], self.size);
    }
  }
}
