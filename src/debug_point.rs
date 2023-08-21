use glium::{VertexBuffer, Display};
use nalgebra_glm::Vec3;

use crate::vertex::Vertex;

#[derive(Debug)]
pub struct DebugPoint {
    pub position: Vec3
}

pub fn cross(display: &Display) -> VertexBuffer<Vertex> {
    VertexBuffer::new(display, &[
        Vertex{position: [-0.2, 0.0, 0.0], normal: [0.0, 1.0, 0.0]},
        Vertex{position: [0.2, 0.0, 0.0], normal: [0.0, 1.0, 0.0]},
        Vertex{position: [0.0, 0.0, -0.2], normal: [0.0, 1.0, 0.0]},
        Vertex{position: [0.0, 0.0, 0.2], normal: [0.0, 1.0, 0.0]},
    ]).expect("Failed to create vertex buffer for debug point")
}
