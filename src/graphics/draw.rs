use std::sync::Arc;
use glam::Vec2;
use image::ImageReader;
use image::RgbaImage;
use crate::graphics::mesh::Mesh;

#[derive(Clone, Debug)]
pub struct Transform2D {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32
}

impl Transform2D {
    pub fn at(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0
        }
    }
}

#[derive(Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const NONE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r.clone(), self.g.clone(), self.b.clone(), self.a.clone()]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [((self.r * 255.0) as u8), ((self.g * 255.0) as u8), ((self.b * 255.0) as u8), ((self.a * 255.0) as u8)]
    }
}

#[derive(Clone, Debug)]
pub enum DrawCommand {
    Mesh2D {
        mesh: Mesh,
        transform: Transform2D,
        image: Option<Arc<Image>>,
        color: Color
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    pub path: String,
    pub image: image::DynamicImage
}

impl Image {

    pub fn from_file(path: &str) -> Self {
        Self {
            path: path.to_string(),
            image: ImageReader::open(path).unwrap().decode().unwrap()
        }
    }

    pub fn single_pixel(color: Color) -> Self {
        let mut img = RgbaImage::new(1, 1);
        img.put_pixel(0, 0, image::Rgba(color.clone().into()));
        Self {
            path: format!("single_pixel_{:?}_{:?}_{:?}_{:?}", color.r, color.g, color.b, color.a),
            image: image::DynamicImage::ImageRgba8(img)
        }
    }
    
    pub fn write_to_file(&self, path: &str) {
        self.image.save(path).unwrap();
    }

}

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

    pub fn end_frame(&mut self) {
        self.commands.clear();
    }

    pub fn set_background_color(&mut self, color: Color) -> &mut Self {
        self.background_color = color;
        self
    }

    pub fn draw_triangle(&mut self, transform: Transform2D, color: Color) -> &mut Self {
        self.commands.push(DrawCommand::Mesh2D {
            mesh: Mesh::new_triangle(),
            image: None,
            transform,
            color
        });
        self
    }

    pub fn draw_rectangle(&mut self, transform: Transform2D, dimension: Vec2, color: Color) -> &mut Self {
        self.commands.push(DrawCommand::Mesh2D {
            mesh: Mesh::new_rectangle(),
            image: None,
            transform,
            color
        });
        self
    }

    pub fn draw_circle(&mut self, transform: Transform2D, radius: f32, segments: u16, color: Color) -> &mut Self {
        self.commands.push(DrawCommand::Mesh2D {
            mesh: Mesh::new_circle(radius, segments),
            image: None,
            transform,
            color
        });
        self
    }

    pub fn draw_image(&mut self, transform: Transform2D, img: Arc<Image>) -> &mut Self {
        self.commands.push(DrawCommand::Mesh2D {
            mesh: Mesh::new_rectangle(),
            image: Some(img),
            transform,
            color: Color::NONE
        });
        self
    }

}