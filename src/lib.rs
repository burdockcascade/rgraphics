pub mod graphics;
pub mod frame;

use std::thread;
use std::time::Duration;
use cgmath::Vector2;
use log::{debug, error};
use crate::frame::Renderer;
use crate::graphics::gpu::Display;
use winit::application::ApplicationHandler;
use winit::event::{DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Debug)]
pub enum InputEvent {
    KeyboardInput(DeviceId, KeyCode),
    CursorMoved(DeviceId, Vector2<f32>)
}

pub trait EventHandler {
    fn on_init(&mut self) {}
    fn on_input_event(&mut self, event: InputEvent) {}
    fn on_update(&mut self, delta: f32) {}
    fn on_draw(&mut self, renderer: &mut Renderer) {}
    fn on_close(&mut self) -> bool { true }
}

pub struct Raymond {
    window_attributes: WindowAttributes,
    display: Option<Display>,
    handler: Box<dyn EventHandler>,
    renderer: Renderer,
    exit_requested: bool,
    elapsed_since_last_frame: f32,
    start: std::time::Instant,
    target_frame_time: Option<f32>
}

impl Raymond {

    pub fn create_window(height : i32, width : i32, title : &str, handler: Box<dyn EventHandler>) -> Self {
        let window_attributes = Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));
        
        Self {
            window_attributes,
            display: None,
            handler,
            elapsed_since_last_frame: 0.0,
            renderer: Renderer::new(),
            start: std::time::Instant::now(),
            target_frame_time: None,
            exit_requested: false
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

    pub fn set_target_fps(&mut self, target: u32) -> &mut Self {
        self.target_frame_time = Some(1.0 / target as f32);
        self
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

        let Some(ref mut display) = self.display else {
            error!("No graphics state found");
            return;
        };

        let window = display.window();

        if window.id() != window_id {
            error!("Window ID mismatch");
            return
        }

        match event {
            WindowEvent::RedrawRequested => {
                
                // start the frame timer
                self.start = std::time::Instant::now();
                
                // call the update handler
                self.handler.on_update(self.elapsed_since_last_frame);
                
                // call the draw handler
                self.handler.on_draw(&mut self.renderer);
                
                // render the frame
                display.render(&mut self.renderer);
                
                // clear the renderer
                self.renderer.end_frame();
                
                // sleep to reach target fps
                if let Some(target_frame_time) = self.target_frame_time {
                    let sleep_time = target_frame_time - self.start.elapsed().as_secs_f32();
                    if sleep_time > 0.0 {
                        thread::sleep(Duration::from_secs_f32(sleep_time));
                    }
                }

                self.elapsed_since_last_frame = self.start.elapsed().as_secs_f32();
                
            }
            WindowEvent::CloseRequested => {
                if self.handler.on_close() {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(physical_size) => {
                display.resize(physical_size);
            }
            WindowEvent::KeyboardInput { device_id, event, .. } => {
                match event.physical_key {
                    PhysicalKey::Code(code) => {

                        // exit on escape
                        if code == KeyCode::Escape {
                            if self.handler.on_close() {
                                self.exit_requested = true;
                            }
                        }

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

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        
        let window = self.display.as_ref().unwrap().window();
        window.request_redraw();
        
        if self.exit_requested {
            event_loop.exit();
        }

        event_loop.set_control_flow(ControlFlow::Poll);

    }
}

