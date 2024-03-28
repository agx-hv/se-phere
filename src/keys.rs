extern crate glfw;
use glfw::{Action, Key, Modifiers, MouseButton};
use num_traits::abs;

pub fn handle_key_event(window: &mut glfw::Window, key: Key, action: Action,modifier:Modifiers, keystates: &mut [i8; 16]) {
    let index = match key { //modular mapping system
        Key::W => 0, // player forward
        Key::A => 1, // player spin left
        Key::S => 2, // player backward
        Key::D => 3, // player spin right
        Key::I => 4, // camera tilt up
        Key::K => 5, // camera tilt down
        Key::Escape => {
            window.set_should_close(true);
            return;
        }
        _ => 999 // everything else
    };
    if index != 999 {
        keystates[index] = if action == Action::Release { 0 } else { 1 };
    }
}
pub fn handle_mouse_button(mouse_button: MouseButton, action: Action,modifier:Modifiers, keystates: &mut [i8; 16]) {
    let index = match mouse_button { //modular mapping system
        MouseButton::Button1 => 10, // leftclick, mesh shit go down
        MouseButton::Button2 => 11, // rightclick,mesh shit go up
        _ => 999 //everything else
    };
    if index != 999 {
        keystates[index] = if action == Action::Release { 0 } else { 1 };
    }
}