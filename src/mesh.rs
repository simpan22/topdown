use std::path::PathBuf;

use glium::{
    uniform, BackfaceCullingMode, Display, DrawParameters, Frame, Program, Surface,
    VertexBuffer, index::NoIndices,
};
use glm::Mat4;

use crate::{light::Light, vertex::Vertex, wavefront};
use nalgebra_glm as glm;

pub struct Mesh<'a> {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub shader: &'a Program,
}

impl<'a> Mesh<'a> {
    pub fn load(display: &Display, path: PathBuf, shader: &'a Program) -> Self {
        let (v_data, _) = wavefront::load(path);
        let v_buffer = VertexBuffer::new(display, &v_data).expect("Failed to create vertex buffer");
        Mesh {
            vertex_buffer: v_buffer,
            shader,
        }
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        model: Mat4,
        view: Mat4,
        projection: Mat4,
        lights: &[&Light],
    ) {
        // // Force type to be compatible with glium's AsUniformValue trait
        // // TODO: Implement AsUniformValue for our own type with a glm::matrix in it.
        let view: [[f32; 4]; 4] = view.into();
        let model: [[f32; 4]; 4] = model.into();
        let projection: [[f32; 4]; 4] = projection.into();

        let light_pos: [f32; 3] = lights[0].position.into();

        frame
            .draw(
                &self.vertex_buffer,
                &NoIndices(glium::index::PrimitiveType::TrianglesList),
                self.shader,
                &uniform! {model: model, view: view, projection: projection, light_pos: light_pos},
                &DrawParameters {
                    // polygon_mode: PolygonMode::Line,
                    backface_culling: BackfaceCullingMode::CullClockwise,
                    ..Default::default()
                },
            )
            .expect("Failed to render");
    }
}
