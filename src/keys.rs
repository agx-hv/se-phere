
extern crate glfw;
use glfw::{Action, Key};

pub fn handle_key_event(window: &mut glfw::Window, key: Key, action: Action, keystates: &mut [i32; 8]) {
    let index = match key { //modular mapping system
        Key::W => 0, // player forward
        Key::A => 1, // player spin left
        Key::S => 2, // player backward
        Key::D => 3, // player spin right
        Key::I => 4, // camera tilt up
        Key::J => 5, //unused
        Key::K => 6, // camera tilt down
        Key::L => 7, //unused
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