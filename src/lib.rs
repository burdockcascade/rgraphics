pub mod graphics;
pub mod frame;

use std::sync::{Arc, Mutex};
use cgmath::{Matrix4, Vector2, Vector3};
use frame::Frame;
use image::DynamicImage;
use log::{debug, error};

use crate::graphics::draw::{Color, DrawCommand, Image};
use crate::graphics::gpu::Display;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

pub trait EventHandler {
    fn on_init(&mut self);
    fn on_keyboard_input(&mut self, key: KeyCode);
    fn on_cursor_moved(&mut self, position: Vector2<f32>);
    fn on_frame(&mut self, frame: &mut Frame);
    fn on_close(&mut self) -> bool;
}

pub struct Raymond {
    window_attributes: WindowAttributes,
    display: Option<Display>,
    handler: Option<Box<dyn EventHandler>>,
    current_frame: Frame,
}

impl Raymond {

    pub fn create_window(height : i32, width : i32, title : &str) -> Self {
        let window_attributes = Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        Self {
            window_attributes,
            display: None,
            handler: None,
            current_frame: Frame::new()
        }
    }

    pub fn with_handler(mut self, handler: Box<dyn EventHandler>) -> Self {
        self.handler = Some(handler);
        self
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

        if let Some(ref mut handler) = self.handler {
            handler.on_init();
        }
        
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
            WindowEvent::CloseRequested => {
                if let Some(ref mut handler) = self.handler {
                    if handler.on_close() {
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::Resized(physical_size) => {
                graphics_state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {

                let frame = &mut self.current_frame;

                if let Some(ref mut handler) = self.handler {
                    handler.on_frame(frame);
                }
                
                graphics_state.set_draw_commands(frame.renderer().commands.clone());
                graphics_state.render().unwrap();
                
                frame.next();
            }
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                print!("Keyboard input: {:?}", device_id);
                
                if let Some(ref mut handler) = self.handler {
                    match event.physical_key {
                        PhysicalKey::Code(code) => {
                            handler.on_keyboard_input(code);
                        },
                        _ => {
                            debug!("Unhandled physical key: {:?}", event.physical_key);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { device_id, position } => {
                if let Some(ref mut handler) = self.handler {
                    handler.on_cursor_moved(Vector2::new(position.x as f32, position.y as f32));
                }
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

