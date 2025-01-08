use cgmath::Matrix4;
use wgpu::Device;
use wgpu::util::DeviceExt;
use crate::graphics::gpu::Vertex;

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

    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

}

#[derive(Clone, Debug)]
pub struct DrawCommand {
    pub mesh: Mesh,
    pub color: Color,
    pub transform: Matrix4<f32>
}

#[derive(Clone, Debug)]
pub struct Mesh {
    vertices: &'static [Vertex],
    pub(crate) indices: &'static [u16],
}

impl Mesh {

    pub fn new_triangle(color: Color) -> Self {
        static VERTICES: [Vertex; 3] = [
            Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0], uv: [0.0, 0.0] },
        ];

        static INDICES: [u16; 3] = [0, 1, 2];

        Self { vertices: &VERTICES, indices: &INDICES }
    }

    pub fn new_square(color: Color) -> Self {
        static VERTICES: [Vertex; 4] = [
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0], uv: [0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0], uv: [0.0, 0.0] },
            Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 1.0, 0.0], uv: [0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0], color: [0.0, 1.0, 0.0], uv: [0.0, 0.0] },
        ];

        static INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

        Self { vertices: &VERTICES, indices: &INDICES }

    }

    pub fn vertex_buffer(&self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn index_buffer(&self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}