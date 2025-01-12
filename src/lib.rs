pub mod graphics;

use cgmath::{Matrix4, Vector2, Vector3};
use image::DynamicImage;
use log::{debug, error};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::graphics::draw::{Color, DrawCommand, Image};
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

    pub fn draw_triangle(&mut self, rotation: f32, color: Color) -> &mut Self {

        // base transformation
        let transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));

        // rotation
        let transform = transform * Matrix4::from_angle_z(cgmath::Rad(rotation));

        self.draw_commands.push(DrawCommand {
            mesh: graphics::draw::Mesh::new_triangle(),
            image: None,
            transform,
            color: color.into()
        });
        self
    }

    pub fn draw_rectangle(&mut self, dimension: Vector2<f32>, position: Vector2<f32>, rotation: f32, color: Color) -> &mut Self {

        // base transformation
        let mut transform = Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0)) * Matrix4::from_nonuniform_scale(dimension.x, dimension.y, 1.0);

        // rotation
        if rotation != 0.0 {
            transform = transform * Matrix4::from_angle_z(cgmath::Rad(rotation));
        }

        self.draw_commands.push(DrawCommand {
            mesh: graphics::draw::Mesh::new_rectangle(),
            image: None,
            transform,
            color: color.into()
        });
        self
    }

    pub fn draw_image(&mut self, img: Image) -> &mut Self {
        self.draw_commands.push(DrawCommand {
            mesh: graphics::draw::Mesh::new_rectangle(),
            image: Some(img),
            transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            color: Color::NONE.into()
        });
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

