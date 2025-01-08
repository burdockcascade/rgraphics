mod graphics;

use cgmath::{Matrix4, Vector3};
use log::{debug, error, info};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::graphics::draw::DrawCommand;
use crate::graphics::gpu::Display;

pub trait EventHandler {
    fn on_init(&mut self);
    fn on_frame(&mut self, delta_time: f64);
    fn on_close(&mut self);
}

pub struct Raymond {
    window_attributes: WindowAttributes,
    draw_commands: Vec<DrawCommand>,
    display: Option<Display>,
}

impl Raymond {

    pub fn create_window(height : i32, width : i32, title : &str) -> Self {
        let window_attributes = Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        Self {
            window_attributes,
            display: None,
            draw_commands: Vec::new(),
        }
    }

    pub fn draw_triangle(&mut self) -> &mut Self {
        self.draw_commands.push(DrawCommand {
            mesh: graphics::draw::Mesh::new_triangle(graphics::draw::Color::GREEN),
            color: graphics::draw::Color::GREEN,
            transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
        });
        self
    }

    pub fn draw_square(&mut self) -> &mut Self {
        self.draw_commands.push(DrawCommand {
            mesh: graphics::draw::Mesh::new_square(graphics::draw::Color::GREEN),
            color: graphics::draw::Color::GREEN,
            transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
        });
        self
    }

    pub fn run(&mut self, event_handler : &mut dyn EventHandler) {
        event_handler.on_init();
        match EventLoop::new() {
            Ok(event_loop) => {
                event_loop.run_app(self).expect("Unable to run app");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        event_handler.on_close();
    }

}

impl ApplicationHandler for Raymond {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(self.window_attributes.clone())
            .unwrap();

        let display = Display::new(window);
        self.display = Some(display);
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
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                graphics_state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                let commands = self.draw_commands.clone();
                graphics_state.set_draw_commands(commands);
                graphics_state.render().unwrap();
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

