
pub struct Keyboard {
    pub is_left: bool,
    pub is_right: bool,
    pub is_up: bool,
    pub is_down: bool,
    pub is_button1: bool,
    pub is_button2: bool,
}

use piston_window::*;

impl Keyboard {
    pub fn update(&mut self, e: &Event) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => self.is_left = true,
                Key::Right => self.is_right = true,
                Key::Up => self.is_up = true,
                Key::Down => self.is_down = true,
                Key::Z => self.is_button1 = true,
                Key::X => self.is_button2 = true,
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Left => self.is_left = false,
                Key::Right => self.is_right = false,
                Key::Up => self.is_up = false,
                Key::Down => self.is_down = false,
                Key::Z => self.is_button1 = false,
                Key::X => self.is_button2 = false,
                _ => {}
            }
        }
    }
}

