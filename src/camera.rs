use std::collections::HashSet;

use glium::glutin::event::VirtualKeyCode;
use hecs::{World, Entity};
use nalgebra_glm::{look_at, vec3, Mat4, Vec3};

pub struct Camera {
    pub view: Mat4,
    pub projection: Mat4,
    pub position: Vec3,
}

fn view_from_pos(pos: Vec3) -> Mat4 {
    look_at(
        &pos,
        &(vec3(pos.x - 10.0, 0.0, pos.z - 10.0)),
        &vec3(0.0, 1.0, 0.0),
    )
}

impl Camera {
    pub fn new(pos: Vec3, near: f32, far: f32) -> Self {
        Camera {
            view: view_from_pos(pos),
            projection: Mat4::new_perspective(
                800.0 / 600.0,
                nalgebra_glm::pi::<f32>() / 4.0,
                near,
                far,
            ),
            position: pos,
        }
    }
    pub fn update_view(&mut self) {
        self.view = view_from_pos(self.position);
    }
}

pub fn camera_system(world: &mut World, pressed_keys: &HashSet<VirtualKeyCode>, camera_entity: Entity) {
    let mut camera = world.get::<&mut Camera>(camera_entity).unwrap();

    if pressed_keys.contains(&VirtualKeyCode::W) {
        camera.position += vec3(-0.001, 0.0, -0.001);
        camera.update_view();
    }

    if pressed_keys.contains(&VirtualKeyCode::S) {
        camera.position += vec3(0.001, 0.0, 0.001);
        camera.update_view();
    }

    if pressed_keys.contains(&VirtualKeyCode::A) {
        camera.position += vec3(-0.001, 0.0, 0.001);
        camera.update_view();
    }

    if pressed_keys.contains(&VirtualKeyCode::D) {
        camera.position += vec3(0.001, 0.0, -0.001);
        camera.update_view();
    }
}
