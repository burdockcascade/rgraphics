use bytemuck::{Pod, Zeroable};
use crate::graphics::draw::Color;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2]
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

// vertex positions for wgpu
// -1.0, 1.0, 0.0, // top left
// 1.0, 1.0, 0.0, // top right
// -1.0, -1.0, 0.0, // bottom left
// 1.0, -1.0, 0.0, // bottom right

impl Mesh {

    pub fn new_triangle() -> Self {
        let vertices = vec![
            Vertex { position: [-0.5, -0.5, 0.0], uv: [0.0, 1.0] }, // bottom left
            Vertex { position: [0.0, 0.5, 0.0], uv: [0.5, 0.0] }, // top
            Vertex { position: [0.5, -0.5, 0.0], uv: [1.0, 1.0] }, // bottom right
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

    pub fn new_circle(radius: f32, segments: u16) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let center = Vertex { position: [0.0, 0.0, 0.0], uv: [0.5, 0.5] };
        vertices.push(center);
        for i in 0..segments {
            let angle = 2.0 * std::f32::consts::PI / segments as f32 * i as f32;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            vertices.push(Vertex { position: [x, y, 0.0], uv: [0.5 + x / 2.0, 0.5 + y / 2.0] });
            if i > 0 {
                indices.push(0);
                indices.push(i + 1);
                indices.push(i);
            }
        }
        indices.push(0);
        indices.push(1);
        indices.push(segments);
        Self { vertices, indices }
    }

}