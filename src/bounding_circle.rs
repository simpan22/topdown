use std::f32::consts::PI;

use nalgebra_glm::Vec2;

use crate::{vertex::Vertex, mesh::Mesh};

#[derive(Copy, Clone, Debug)]
pub struct BoundingCircle {
    pub r: f32,
    pub ground_pos: Vec2,
}

impl BoundingCircle {
    pub fn from_mesh(mesh: &Mesh) -> Self {
        let projected: Vec<Vec2> = mesh
            .vertices
            .iter()
            .map(|v| Vec2::new(v.position[0], v.position[2]))
            .collect();

        let midpoint = projected
            .clone()
            .into_iter()
            .reduce(|a: Vec2, b: Vec2| (a + b) / (projected.len() as f32))
            .unwrap();

        let mut max_r = 0.0;
        for v in projected {
            let diff = midpoint - v;
            let r = diff.norm();
            if r > max_r {
                max_r = r;
            }
        }

        BoundingCircle {
            r: max_r * 1.2,
            ground_pos: midpoint,
        }

    }
    pub fn triangle_strip(&self, n_segments: u32, width: f32) -> Vec<Vertex> {
        let segment_angle = (2.0 * PI) / (n_segments as f32);

        let mut vertices = vec![];
        for i in 0..(n_segments + 1) {
            // Inner circle
            vertices.push(Vertex {
                position: [
                    self.ground_pos.x + self.r * f32::cos(segment_angle * i as f32),
                    0.0,
                    self.ground_pos.y + self.r * f32::sin(segment_angle * i as f32),
                ],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0]
            });

            // Outer circle
            vertices.push(Vertex {
                position: [
                    self.ground_pos.x + (self.r + width) * f32::cos(segment_angle * i as f32),
                    0.0,
                    self.ground_pos.y + (self.r + width) * f32::sin(segment_angle * i as f32),
                ],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0]
            });
        }

        vertices
    }
}
