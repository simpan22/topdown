use std::f32::consts::PI;

use nalgebra_glm::{Mat4, Vec3};

#[derive(Debug, Clone)]
pub struct Transformation {
    pub pos: Vec3,
    pub rotation: f32,
    pub scale: f32,
}

fn calculate_model(pos: Vec3, rot: f32, scale: f32) -> Mat4 {
    Mat4::new_translation(&pos)
        * Mat4::new_scaling(scale)
        * Mat4::new_rotation(Vec3::y() * (rot - PI/2.0))
}

impl Transformation {

    pub fn model(&self) -> Mat4 {
        calculate_model(self.pos, self.rotation, self.scale)
    }

    pub fn new(pos: Vec3, rot: f32, scale: f32) -> Self {
        Transformation {
            pos: pos,
            rotation: rot,
            scale: scale,
        }
    }

    pub fn translation(pos: Vec3) -> Self {
        Transformation {
            pos: pos,
            rotation: 0.0,
            scale: 1.0,
        }
    }
}
