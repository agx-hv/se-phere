
extern crate glfw;
use glfw::{Action, Context, Key};

pub fn handle_key_event(window: &mut glfw::Window, key: Key, action: Action, keystates: &mut [i32; 8]) {
    let index = match key {
        Key::W => 0, //modular mapping system
        Key::A => 1,
        Key::S => 2,
        Key::D => 3,
        Key::I => 4,
        Key::J => 5,
        Key::K => 6,
        Key::L => 7,
        Key::Escape => {
            window.set_should_close(true);
            return;
        }
        _ => {
            999
        }
    };
    if index != 999 {
        keystates[index] = if action == Action::Release { 0 } else { 1 };
    }
}