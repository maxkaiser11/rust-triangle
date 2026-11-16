use winit::event::{self, Event, WindowEvent};
use winit::event_loop::{self, EventLoop};
use winit::platform::wayland;
use winit::window::{Window, WindowBuilder};

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Shader Playground")
        .build(&event_loop)
        .unwrap();

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent { ref event, window_id }
                    if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => control_flow.exit(),
                        _ => {}
                    }
                }
            _ => {}
        }
    });
}
