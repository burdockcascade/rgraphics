pub mod graphics;
pub mod frame;

use cgmath::Vector2;
use log::{debug, error, info};
use crate::frame::Renderer;
use crate::graphics::draw::{Color, DrawCommand, Image};
use crate::graphics::gpu::Display;
use winit::application::ApplicationHandler;
use winit::event::{DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Debug)]
pub enum InputEvent {
    KeyboardInput(DeviceId, KeyCode),
    CursorMoved(DeviceId, Vector2<f32>)
}

pub trait EventHandler {
    fn on_init(&mut self);
    fn on_input_event(&mut self, event: InputEvent);
    fn on_update(&mut self, delta: f32);
    fn on_draw(&mut self, renderer: &mut Renderer);
    fn on_close(&mut self) -> bool;
}

const DEFAULT_FPS: f32 = 60.0;

pub struct Raymond {
    window_attributes: WindowAttributes,
    display: Option<Display>,
    handler: Box<dyn EventHandler>,
    renderer: Renderer,
    min_frame_duration: f32,
}

impl Raymond {

    pub fn create_window(height : i32, width : i32, title : &str, handler: Box<dyn EventHandler>) -> Self {
        let window_attributes = Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));
        
        let min_frame_duration = 1.0 / DEFAULT_FPS;
        
        Self {
            window_attributes,
            display: None,
            handler,
            min_frame_duration,
            renderer: Renderer::default(),
        }
    }

    pub fn run(&mut self) {
        match EventLoop::new() {
            Ok(event_loop) => {
                event_loop.run_app(self).expect("Unable to run app");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

}

impl ApplicationHandler for Raymond {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(self.window_attributes.clone())
            .unwrap();

        let display = Display::new(window);
        self.display = Some(display);
        
        self.handler.on_init();
        
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

        let Some(ref mut graphics_state) = self.display else {
            error!("No graphics state found");
            return;
        };

        let window = graphics_state.window();

        if window.id() != window_id {
            error!("Window ID mismatch");
            return
        }

        match event {
            WindowEvent::RedrawRequested => {
                
                // start the frame timer
                let start = std::time::Instant::now();
                
                // call the update handler
                self.handler.on_update(0.0);
                
                // call the draw handler
                self.handler.on_draw(&mut self.renderer);
                
                // render the frame
                graphics_state.set_draw_commands(self.renderer.commands.clone());
                graphics_state.render().unwrap();
                
                // clear the draw commands
                self.renderer.commands.clear();
                
                // calculate fps and print
                let fps = (1.0 / (start.elapsed().as_secs_f32())) as u32;
                info!("FPS: {}", fps);
                
            }
            WindowEvent::CloseRequested => {
                if self.handler.on_close() {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(physical_size) => {
                graphics_state.resize(physical_size);
            }
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                match event.physical_key {
                    PhysicalKey::Code(code) => {
                        self.handler.on_input_event(InputEvent::KeyboardInput(device_id, code));
                    },
                    _ => {
                        debug!("Unhandled physical key: {:?}", event.physical_key);
                    }
                }
            }
            WindowEvent::CursorMoved { device_id, position } => {
                self.handler.on_input_event(InputEvent::CursorMoved(device_id, Vector2::new(position.x as f32, position.y as f32)));
            }
            _ => {
                debug!("Unhandled window event: {:?}", event);
            }
        }

    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.display.as_ref().unwrap().window();
        window.request_redraw();
    }
}

