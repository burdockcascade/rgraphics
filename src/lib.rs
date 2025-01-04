mod gpu;
mod application;

use crate::application::ApplicationState;

use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;

pub trait EventHandler {
    fn on_init(&mut self);
    fn on_frame(&mut self, delta_time: f64);
    fn on_close(&mut self);
}

pub struct IgniteFX {
    window_state : ApplicationState
}

impl IgniteFX {

    pub fn new(height : i32, width : i32, title : &str) -> Self {
        IgniteFX {
            window_state: ApplicationState::new(height, width, title)
        }
    }

    pub fn run(&mut self, event_handler : &mut dyn EventHandler) {
        event_handler.on_init();
        match EventLoop::new() {
            Ok(event_loop) => {
                event_loop.run_app(&mut self.window_state).expect("Unable to run app");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        event_handler.on_close();
    }

}

