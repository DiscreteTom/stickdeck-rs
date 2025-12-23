use stickdeck_common::{Mouse, MouseButton};
use windows::Win32::UI::Input::KeyboardAndMouse::{
  SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
  MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEINPUT,
  MOUSE_EVENT_FLAGS,
};

pub struct MouseController {
  last_button_state: MouseButton,
}

impl MouseController {
  const fn empty_mouse_input() -> INPUT {
    INPUT {
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
    }
  }

  const INPUT_SIZE: i32 = std::mem::size_of_val(&Self::empty_mouse_input()) as i32;

  pub fn new() -> Self {
    let last_button_state = MouseButton::default();
    Self { last_button_state }
  }

  /// Apply the mouse state.
  pub fn apply(&mut self, data: &Mouse) {
    let mut input = Self::empty_mouse_input();

    unsafe {
      // handle mouse move
      if data.x != 0 || data.y != 0 {
        input.Anonymous.mi.dx = data.x as i32;
        input.Anonymous.mi.dy = data.y as i32;
        input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_MOVE.0;
      }

      // handle mouse button
      if data.buttons != self.last_button_state {
        if data.buttons.is_left_button_down() != self.last_button_state.is_left_button_down() {
          if data.buttons.is_left_button_down() {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_LEFTDOWN.0;
          } else {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_LEFTUP.0;
          }
        }

        if data.buttons.is_right_button_down() != self.last_button_state.is_right_button_down() {
          if data.buttons.is_right_button_down() {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_RIGHTDOWN.0;
          } else {
            input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_RIGHTUP.0;
          }
        }

        self.last_button_state = data.buttons;
      }

      // handle scroll
      if data.scroll != 0 {
        input.Anonymous.mi.mouseData = data.scroll as u32;
        input.Anonymous.mi.dwFlags.0 |= MOUSEEVENTF_WHEEL.0;
      }

      SendInput(&[input], Self::INPUT_SIZE);
    }
  }
}
