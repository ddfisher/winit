#![allow(clippy::single_match)]

use simple_logger::SimpleLogger;
use winit::{
    event::{ElementState, Event, KeyEvent, MouseButton, StartCause, WindowEvent},
    event_loop::EventLoop,
    keyboard::Key,
    window::{Window, WindowId},
};

#[path = "util/fill.rs"]
mod fill;

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new().unwrap();

    let window_1 = Window::builder().build(&event_loop).unwrap();
    let window_2 = Window::builder().build(&event_loop).unwrap();

    let mut switched = false;
    let mut entered_id = window_2.id();
    let mut cursor_location = None;

    event_loop.run(move |event, elwt| match event {
        Event::NewEvents(StartCause::Init) => {
            eprintln!("Switch which window is to be dragged by pressing \"x\".")
        }
        Event::WindowEvent { event, window_id } => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::CursorMoved { position, .. } => cursor_location = Some(position),
            WindowEvent::MouseInput { state, button, .. } => {
                let window = if (window_id == window_1.id() && switched)
                    || (window_id == window_2.id() && !switched)
                {
                    &window_2
                } else {
                    &window_1
                };

                match (button, state) {
                    (MouseButton::Left, ElementState::Pressed) => window.drag_window().unwrap(),
                    (MouseButton::Right, ElementState::Released) => {
                        if let Some(position) = cursor_location {
                            window.show_window_menu(position);
                        }
                    }
                    _ => (),
                }
            }
            WindowEvent::CursorEntered { .. } => {
                entered_id = window_id;
                name_windows(entered_id, switched, &window_1, &window_2)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        logical_key: Key::Character(c),
                        ..
                    },
                ..
            } => match c.as_str() {
                "x" => {
                    switched = !switched;
                    name_windows(entered_id, switched, &window_1, &window_2);
                    println!("Switched!")
                }
                "d" => {
                    let window = if (window_id == window_1.id() && switched)
                        || (window_id == window_2.id() && !switched)
                    {
                        &window_2
                    } else {
                        &window_1
                    };

                    window.set_decorations(!window.is_decorated());
                }
                _ => (),
            },
            WindowEvent::RedrawRequested => {
                if window_id == window_1.id() {
                    fill::fill_window(&window_1);
                } else if window_id == window_2.id() {
                    fill::fill_window(&window_2);
                }
            }
            _ => (),
        },

        _ => (),
    })
}

fn name_windows(window_id: WindowId, switched: bool, window_1: &Window, window_2: &Window) {
    let (drag_target, other) =
        if (window_id == window_1.id() && switched) || (window_id == window_2.id() && !switched) {
            (&window_2, &window_1)
        } else {
            (&window_1, &window_2)
        };
    drag_target.set_title("drag target");
    other.set_title("winit window");
}
