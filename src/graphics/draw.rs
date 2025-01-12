use crate::graphics::gpu::Vertex;
use cgmath::Matrix4;
use image::ImageReader;
use wgpu::util::DeviceExt;
use wgpu::Device;

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
pub struct DrawCommand {
    pub mesh: Mesh,
    pub image: Option<Image>,
    pub transform: Matrix4<f32>,
    pub color: Color
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
    
    pub fn write_to_file(&self, path: &str) {
        self.image.save(path).unwrap();
    }

}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

impl Mesh {

    pub fn new_triangle() -> Self {
        let vertices = vec![
            Vertex { position: [0.0, 1.0, 0.0], uv: [0.0, 0.0] },
            Vertex { position: [-1.0, -1.0, 0.0], uv: [0.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0], uv: [0.0, 0.0] },
        ];
        let indices = vec![0, 1, 2];
        Self { vertices, indices }
    }

    pub fn new_rectangle() -> Self {
        let vertices = vec![
            Vertex { position: [-0.5, 0.5, 0.0], uv: [0.0, 0.0] }, // top left
            Vertex { position: [0.5, 0.5, 0.0], uv: [1.0, 0.0] }, // top right
            Vertex { position: [-0.5, -0.5, 0.0], uv: [0.0, 1.0] }, // bottom left
            Vertex { position: [0.5, -0.5, 0.0], uv: [1.0, 1.0] }, // bottom right
        ];
        let indices = vec![
            0, 1, 2, // first triangle
            2, 1, 3, // second triangle
        ];
        Self { vertices, indices }
    }

    pub fn vertex_buffer(&self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(self.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn index_buffer(&self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(self.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}