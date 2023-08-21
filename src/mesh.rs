use std::path::PathBuf;

use glium::{texture::SrgbTexture2d, Display, VertexBuffer};
use nalgebra_glm::Vec3;

use crate::{
    texture::{load_from, sample_texture},
    vertex::Vertex,
    wavefront,
};

pub struct Mesh {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub color: Vec3,
    pub vertices: Vec<Vertex>,
    pub texture: SrgbTexture2d,
}

impl Mesh {
    pub fn load(display: &Display, path: PathBuf, color: Vec3) -> Self {
        let (v_data, _) = wavefront::load(path);
        let v_buffer = VertexBuffer::new(display, &v_data).expect("Failed to create vertex buffer");

        let texture = SrgbTexture2d::new(display, load_from("tank_texture.png".into())).unwrap();
        println!("{:?}, {:?}", texture.width(), texture.height());
        Mesh {
            vertex_buffer: v_buffer,
            color,
            vertices: v_data,
            texture,
        }
    }

    pub fn new_floor(display: &Display, color: Vec3) -> Self {
        let v_data = vec![
            Vertex {
                position: [10.0, 0.0, -10.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [1.0, 0.0],
            },
            Vertex {
                position: [10.0, 0.0, 10.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [1.0, 1.0],
            },
            Vertex {
                position: [-10.0, 0.0, -10.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0],
            },
            Vertex {
                position: [-10.0, 0.0, -10.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0],
            },
            Vertex {
                position: [10.0, 0.0, 10.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [1.0, 1.0],
            },
            Vertex {
                position: [-10.0, 0.0, 10.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 1.0],
            },
        ];

        let v_buffer = VertexBuffer::new(display, &v_data).expect("Failed to create vertex buffer");
        Mesh {
            vertex_buffer: v_buffer,
            color,
            vertices: v_data,
            texture: SrgbTexture2d::new(display, sample_texture()).unwrap(),
        }
    }
}
