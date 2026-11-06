use std::error::Error;
use winit::application::ApplicationHandler;
use winit::error::OsError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

#[derive(Default)]
struct App {
    window: Option<Window>,
    startup_error: Option<OsError>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        match event_loop.create_window(Window::default_attributes().with_title("Ash")) {
            Ok(window) => self.window = Some(window),
            Err(error) => {
                self.startup_error = Some(error);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if matches!(event, WindowEvent::CloseRequested)
            && self.window.as_ref().is_some_and(|window| window.id() == id)
        {
            event_loop.exit();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();
    EventLoop::new()?.run_app(&mut app)?;
    app.startup_error.map_or(Ok(()), |error| Err(error.into()))
}
