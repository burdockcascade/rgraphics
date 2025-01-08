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
    pub transform: Matrix4<f32>
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

impl Mesh {

    pub fn new_triangle(color: Color) -> Self {

        let color = [color.r, color.g, color.b];

        let vertices = vec![
            Vertex { position: [0.0, 1.0, 0.0], color, uv: [0.0, 0.0] },
            Vertex { position: [-1.0, -1.0, 0.0], color, uv: [0.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0], color, uv: [0.0, 0.0] },
        ];

        let indices = vec![0, 1, 2];

        Self { vertices, indices }
    }

    pub fn new_rectangle(color: Color) -> Self {

        let color = [color.r, color.g, color.b];

        let vertices = vec![
            Vertex { position: [-1.0, -1.0, 1.0], color, uv: [0.0, 0.0] },
            Vertex { position: [1.0, -1.0, 1.0], color, uv: [0.0, 0.0] },
            Vertex { position: [1.0, 1.0, 1.0], color, uv: [0.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 1.0], color, uv: [0.0, 0.0] },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

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