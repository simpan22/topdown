use std::path::PathBuf;

use glium::{Display, VertexBuffer};

use crate::{vertex::Vertex, wavefront};

pub struct Mesh {
    pub vertex_buffer: VertexBuffer<Vertex>,
}

impl Mesh {
    pub fn load(display: &Display, path: PathBuf) -> Self {
        let (v_data, _) = wavefront::load(path);
        let v_buffer = VertexBuffer::new(display, &v_data).expect("Failed to create vertex buffer");
        Mesh {
            vertex_buffer: v_buffer,
        }
    }
}
