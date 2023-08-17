use nalgebra_glm::Vec3;

pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
}

impl Light {
    pub fn new(pos: Vec3, color: Vec3) -> Self {
        Light {
            position: pos,
            color,
        }
    }
}
