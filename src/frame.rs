use std::sync::Arc;
use crate::graphics::draw::{Color, DrawCommand, Image, Mesh};
use cgmath::{InnerSpace, Matrix4, Vector2, Vector3};
use crate::graphics::gpu::Display;

pub struct Renderer {
    pub commands: Vec<DrawCommand>,
    pub background_color: Color
}

impl Renderer {
    
    pub fn new() -> Self {
        Self {
            commands: Vec::with_capacity(8),
            background_color: Color::BLACK
        }
    }
    
    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }
    
    pub fn set_background_color(&mut self, color: Color) -> &mut Self {
        self.background_color = color;
        self
    }

    pub fn draw_triangle(&mut self, position: Vector2<f32>, color: Color) -> &mut Self {

        // base transformation
        let transform = Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0));

        self.commands.push(DrawCommand {
            mesh: Mesh::new_triangle(),
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

        self.commands.push(DrawCommand {
            mesh: Mesh::new_rectangle(),
            image: None,
            transform,
            color: color.into()
        });
        self
    }
    
    pub fn draw_circle(&mut self, position: Vector2<f32>, radius: f32, segments: u16, color: Color) -> &mut Self {
        self.commands.push(DrawCommand {
            mesh: Mesh::new_circle(radius, segments),
            image: None,
            transform: Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0)),
            color: color.into()
        });
        self
    }
    
    pub fn draw_line(&mut self, start: Vector2<f32>, end: Vector2<f32>, thickness: f32, color: Color) -> &mut Self {
        let direction = end - start;
        let length = direction.magnitude();
        let angle = direction.y.atan2(direction.x);
        let transform = Matrix4::from_translation(Vector3::new(start.x, start.y, 0.0)) * Matrix4::from_nonuniform_scale(length, thickness, 1.0) * Matrix4::from_angle_z(cgmath::Rad(angle));
        self.commands.push(DrawCommand {
            mesh: Mesh::new_rectangle(),
            image: None,
            transform,
            color: color.into()
        });
        self
    }

    pub fn draw_image(&mut self, position: Vector2<f32>, img: Arc<Image>) -> &mut Self {
        self.commands.push(DrawCommand {
            mesh: Mesh::new_rectangle(),
            image: Some(img),
            transform: Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0)),
            color: Color::NONE.into()
        });
        self
    }
    
    // get commands and then clear
    pub fn take_commands(&mut self) -> Vec<DrawCommand> {
        std::mem::take(&mut self.commands)
    }

}