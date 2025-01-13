use crate::graphics::draw::{Color, DrawCommand, Image, Mesh};
use cgmath::{Matrix4, Vector2, Vector3};

pub struct Frame {
    count: usize,
    renderer: Renderer,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            count: 0,
            renderer: Renderer::default()
        }
    }
    
    pub fn next(&mut self) {
        self.count += 1;
        self.renderer.commands.clear();
    }
    
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}

pub struct Renderer {
    pub(crate) commands: Vec<DrawCommand>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            commands: Vec::new()
        }
    }
}

impl Renderer {

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

    pub fn draw_image(&mut self, position: Vector2<f32>, img: Image) -> &mut Self {
        self.commands.push(DrawCommand {
            mesh: Mesh::new_rectangle(),
            image: Some(img),
            transform: Matrix4::from_translation(Vector3::new(position.x, position.y, 0.0)),
            color: Color::NONE.into()
        });
        self
    }

}