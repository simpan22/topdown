use hecs::{World, Entity};

use crate::{bounding_circle::BoundingCircle, mouse::Cursor, transformation::Transformation};

#[derive(Debug, Clone)]
pub struct Selectable {
    pub selected: bool,
    pub hover: bool,
    pub bounding_circle: BoundingCircle,
}

impl Selectable {
    pub fn new(bounding_circle: BoundingCircle) -> Self {
        Self {
            selected: false,
            hover: false,
            bounding_circle,
        }
    }
}

pub fn select_system(world: &mut World, cursor_entity: Entity) {
    // let mut cursors = world.query::<(&Cursor,)>();
    // let cursor = cursors.iter().last();
    //
    let cursor = world.get::<&Cursor>(cursor_entity);
    if let Ok(cursor) = cursor {
        world
            .query::<(&mut Selectable, &Transformation)>()
            .iter()
            .for_each(|(_, (selectable, transformation))| {
                let cursor_pos = cursor.position.xz();
                let bc_pos = selectable.bounding_circle.ground_pos + transformation.pos.xz();
                let world_r = selectable.bounding_circle.r * transformation.scale;

                let diff = bc_pos - cursor_pos;
                if diff.norm() < world_r {
                    selectable.hover = true;
                } else {
                    selectable.hover = false;
                }
            });
    }
}
