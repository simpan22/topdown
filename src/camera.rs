use nalgebra_glm::{Mat4, Vec3, look_at};

pub struct Camera {
    pub view: Mat4,
    pub projection: Mat4,
}

impl Camera {
    pub fn new(pos: Vec3, center: Vec3, near: f32, far: f32) -> Self {
        Camera {
            // view: Mat4::new_translation(&pos),
            view: look_at(&pos, &center, &Vec3::new(0.0, 1.0, 0.0)),
            projection: Mat4::new_perspective(800.0/600.0, nalgebra_glm::pi::<f32>() / 4.0, near, far),
        }
    }
}
