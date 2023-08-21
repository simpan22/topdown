use std::collections::HashSet;

use glium::{
    glutin::{
        dpi::PhysicalPosition,
        event::{ElementState, ModifiersState, MouseButton, MouseScrollDelta},
    },
    Display, VertexBuffer,
};
use hecs::{Entity, World};
use nalgebra_glm::{Vec2, Vec3, Vec4};

use crate::{camera::Camera, movement::Movement, selectable::Selectable, vertex::Vertex};

pub struct MouseButtonState {
    pub left_pressed: bool,
    pub right_pressed: bool,
}

pub struct Mouse {
    screen_pos: Vec2,
    max_width: u32,
    max_height: u32,
    pub button_state: MouseButtonState,
}

impl Mouse {
    pub fn new(screen_w: u32, screen_h: u32) -> Self {
        Mouse {
            screen_pos: Vec2::new((screen_w / 2) as f32, (screen_h / 2) as f32),
            max_width: screen_w,
            max_height: screen_h,
            button_state: MouseButtonState {
                left_pressed: false,
                right_pressed: false,
            },
        }
    }

    pub fn update(&mut self, pos: &PhysicalPosition<f64>) {
        self.screen_pos.x = pos.x as f32;
        self.screen_pos.y = pos.y as f32;
    }
}

// Entity tag component
pub struct Cursor {
    pub position: Vec3,
}

pub fn cursor_system(
    mouse: &Mouse,
    world: &mut World,
    cursor_entity: Entity,
    camera_entity: Entity,
) {
    let camera = world.get::<&Camera>(camera_entity).unwrap();
    let mut cursor = world.get::<&mut Cursor>(cursor_entity).unwrap();

    let ndc = Vec3::new(
        ((mouse.screen_pos.x as f32) / (mouse.max_width as f32)) * 2.0 - 1.0,
        1.0 - ((mouse.screen_pos.y as f32) / (mouse.max_height as f32)) * 2.0,
        0.0,
    );

    // Matrices needed
    let inv_projection = camera
        .projection
        .try_inverse()
        .expect("Failed to invert projection matrix");
    let inv_view = camera
        .view
        .try_inverse()
        .expect("Failed to invert view matrix");

    let hcc = Vec4::new(ndc.x, ndc.y, -1.0, 1.0);

    let mut eye = inv_projection * hcc;
    eye.w = 0.0;

    let ray = inv_view * eye;
    let dir = ray.xyz().normalize();

    let t = -camera.position.y / dir.y;
    let x = camera.position.x + dir.x * t;
    let z = camera.position.z + dir.z * t;
    let y = 0.0;

    cursor.position.x = x;
    cursor.position.y = y;
    cursor.position.z = z;
}

pub fn create_cursor_vb(display: &Display) -> VertexBuffer<Vertex> {
    VertexBuffer::new(
        display,
        &[
            Vertex {
                position: [-0.2, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0],
            },
            Vertex {
                position: [0.2, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, -0.2],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.2],
                normal: [0.0, 1.0, 0.0],
                texture_coord: [0.0, 0.0],
            },
        ],
    )
    .expect("Failed to create vertex buffer for debug point")
}

pub fn mouse_click_system(
    world: &mut World,
    selected: &mut HashSet<Entity>,
    button: MouseButton,
    state: ElementState,
    modifiers: ModifiersState,
    cursor_entity: Entity,
) {
    match button {
        MouseButton::Left => {
            if state == ElementState::Released {
                for (id, (selectable,)) in world.query_mut::<(&mut Selectable,)>() {
                    if selectable.hover {
                        selectable.selected = true;
                        selected.insert(id);
                    } else {
                        if !modifiers.shift() {
                            selectable.selected = false;
                            selected.remove(&id);
                        }
                    }
                }
            }
        }
        MouseButton::Right => {
            if state == ElementState::Released {
                let cursor = world.get::<&Cursor>(cursor_entity).unwrap();
                world
                    .query::<(&mut Movement, &Selectable)>()
                    .iter()
                    .for_each(|(_, (movement, selectable))| {
                        if selectable.selected {
                            movement.target_pos = Some(cursor.position.xz());
                        }
                    });
            }
        }
        _ => {}
    }
}

pub fn mouse_scroll_system(world: &mut World, delta: MouseScrollDelta, camera_entity: Entity) {
    match delta {
        MouseScrollDelta::LineDelta(_, y) => {
            let mut camera = world.get::<&mut Camera>(camera_entity).unwrap();
            camera.position.y -= y;
            camera.update_view();
        }
        MouseScrollDelta::PixelDelta(PhysicalPosition { .. }) => {
            panic!("Scroll only implemented for line based input devices");
        }
    }
}
