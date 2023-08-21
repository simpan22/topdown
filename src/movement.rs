use hecs::World;
use nalgebra_glm::{Vec2, vec3};

use crate::transformation::Transformation;

pub struct Movement {
    pub target_pos: Option<Vec2>,
}

pub fn movement_system(world: &mut World) {
    for (_, (movement, transformation)) in world.query_mut::<(&Movement, &mut Transformation)>() {
        if let Some(target_pos) = movement.target_pos {
            let target_diff = target_pos - transformation.pos.xz();
            if target_diff.norm() > 0.5 {
                // TODO: Handle wraparound angles
                let target_angle = f32::atan2(-target_diff.y, target_diff.x);
                let angle_diff = target_angle - transformation.rotation;

                if angle_diff.abs() < 0.1 {
                    transformation.pos += vec3(f32::cos(transformation.rotation), 0.0, -f32::sin(transformation.rotation)) * 0.001;
                } else {
                    let angle_diff = angle_diff / angle_diff.abs();
                    transformation.rotation += angle_diff * 0.001;
                }
            }
        }
    }
}
