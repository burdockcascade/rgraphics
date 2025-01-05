use wgpu::Device;
use wgpu::util::DeviceExt;
use crate::graphics::gpu::Vertex;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];

pub enum Color {
    Red,
    Green,
    Blue,
    Silver,
    White,
}

impl Color {
    pub fn to_rgba(&self) -> [f32; 3] {
        match self {
            Color::Red => [1.0, 0.0, 0.0],
            Color::Green => [0.0, 1.0, 0.0],
            Color::Blue => [0.0, 0.0, 1.0],
            Color::Silver => [0.75, 0.75, 0.75],
            Color::White => [1.0, 1.0, 1.0],
        }
    }
}

pub struct Mesh {
    vertices: &'static [Vertex],
    pub(crate) indices: &'static [u16],
}

impl Mesh {

    pub fn triangle(color: Color) -> Self {
        static VERTICES: [Vertex; 3] = [
            Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
        ];

        static INDICES: [u16; 3] = [0, 1, 2];

        Self { vertices: &VERTICES, indices: &INDICES }
    }

    pub fn square(color: Color) -> Self {
        static VERTICES: [Vertex; 4] = [
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.0], color: [0.0, 1.0, 0.0] },
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