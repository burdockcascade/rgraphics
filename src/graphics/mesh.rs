use image::math::Rect;
use lyon::math::Point;
use lyon::path::traits::PathBuilder;
use lyon::tessellation::{StrokeOptions, StrokeTessellator, StrokeVertex};
use crate::graphics::gpu::Vertex;
use lyon::lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};
use lyon::path::{Path, Winding};
use lyon::math::{point, Box2D};

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

    pub fn new_polygon(vertices: Vec<Point>) -> Self {
        let mut builder = Path::builder();
        builder.begin(vertices[0]);
        for vertex in vertices.iter().skip(1) {
            builder.line_to(*vertex);
        }
        builder.close();
        Self::new_filled_path(builder.build())
    }

    pub fn new_triangle() -> Self {
        Self::new_polygon(vec![
            Point::new(0.0, 0.5),
            Point::new(0.5, -0.5),
            Point::new(-0.5, -0.5)
        ])
    }

    pub fn new_rectangle(width: f32, height: f32) -> Self {
        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();
        tessellator.tessellate_rectangle(  
            &Box2D {
                min: point(-1.0, -1.0),
                max: point(1.0, 1.0),
            },
            &FillOptions::DEFAULT,
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                Vertex {
                    position: [vertex.position().x, vertex.position().y, 0.0],
                    uv: [0.0, 0.0],
                }
            }),
        ).unwrap();

        let indices = geometry.indices.clone();
        let vertices = geometry.vertices.clone();

        Self { vertices, indices }

    }

    pub fn new_circle(radius: f32, segments: u16) -> Self {
        let mut builder  = Path::builder();
        builder.add_circle(point(0.0, 0.0), radius, Winding::Positive);
        builder.close();
        Self::new_filled_path(builder.build())
    }

    pub fn new_filled_path(path: Path) -> Mesh {
        // Create a destination vertex and index buffers.
        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();

        // Create the destination tessellator.
        let mut tessellator = FillTessellator::new();

        tessellator.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                Vertex {
                    position: [vertex.position().x, vertex.position().y, 0.0],
                    uv: [0.0, 0.0],
                }
            }),
        ).unwrap();

        let indices = geometry.indices.clone();
        let vertices = geometry.vertices.clone();

        Self { vertices, indices }
    }

}
