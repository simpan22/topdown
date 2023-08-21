extern crate glium;

use std::collections::HashSet;

use bounding_circle::BoundingCircle;
use camera::{camera_system, Camera};
use glium::glutin::event::{self, VirtualKeyCode};
use glium::{glutin::event_loop::EventLoop, Display};
use hecs::{Entity, World};
use light::Light;
use mesh_repo::MeshRepo;
use mouse::{cursor_system, mouse_click_system, Cursor, Mouse, mouse_scroll_system};
use movement::{movement_system, Movement};
use nalgebra_glm::{vec3, Vec3};
use render::render_system;
use selectable::{select_system, Selectable};
use transformation::Transformation;

pub mod bounding_circle;
pub mod camera;
pub mod light;
pub mod math;
pub mod mesh;
pub mod mesh_repo;
pub mod mouse;
pub mod render;
pub mod movement;
pub mod selectable;
pub mod shader;
pub mod transformation;
pub mod vertex;
pub mod wavefront;
pub mod texture;

static WIDTH: u32 = 1024;
static HEIGHT: u32 = 768;

// Simple system that rotate its entities around the y-axis
struct Rotate {}
fn rotate_system(world: &mut World) {
    for (_, (transformation, _)) in world.query_mut::<(&mut Transformation, &Rotate)>() {
        transformation.rotation += 0.0001;
    }
}

fn initialize_glium(w: u32, h: u32) -> (Display, EventLoop<()>) {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(w, h))
        .with_title("Topdown");

    let cb = glium::glutin::ContextBuilder::new();
    let gl_window = cb.build_windowed(wb, &event_loop).unwrap();
    gl_window.window().set_cursor_visible(false);
    let display = glium::Display::from_gl_window(gl_window).unwrap();

    (display, event_loop)
}

fn main() {
    let (display, event_loop) = initialize_glium(WIDTH, HEIGHT);
    let mut mouse = Mouse::new(WIDTH, HEIGHT);

    // Create the world
    let mut world = World::new();

    // Set up mesh repository and load shaders
    let mut mesh_repo = MeshRepo::new();
    let shader = shader::load(&display, "fragment.glsl".into(), "vertex.glsl".into());

    // Floor
    let mesh = mesh::Mesh::new_floor(&display, Vec3::new(0.2, 0.1, 0.1));
    let floor_mesh = mesh_repo.insert(mesh);

    // Load an example mesh and add to mesh repo (cube)
    let mesh = mesh::Mesh::load(&display, "tank.obj".into(), Vec3::new(0.0, 0.3, 0.0));
    let select_circle = BoundingCircle::from_mesh(&mesh);
    let tank_mesh = mesh_repo.insert(mesh);

    // Camera
    let camera_entity = world.spawn((Camera::new(
        Vec3::new(10.0, 30.0, 10.0), // eye
        0.1,
        50.0,
    ),));

    // Light
    world.spawn((
        Light::new(
            Vec3::new(-5.0, 5.0, 5.0), // Pos
            Vec3::new(1.0, 1.0, 1.0),  // Color
        ),
    ));

    // Terrain
    world.spawn((
        floor_mesh.clone(),
        Transformation::new(vec3(0.0, 0.0, 0.0), 0.0, 1.0),
    ));

    // Box
    for i in 0..3 {
        world.spawn((
            tank_mesh.clone(),
            Transformation::new(vec3(-5.0 + (i as f32) * 5.0, 0.0, 0.0), 0.0, 0.2),
            Selectable::new(select_circle),
            Movement{target_pos: None}
        ));
    }

    // Spawn a mouse cursor
    let cursor_entity = world.spawn((Cursor {
        position: vec3(0.5, 0.0, 0.5),
    },));

    // Selected entities
    let mut selected: HashSet<Entity> = HashSet::new();
    let mut pressed_keys: HashSet<VirtualKeyCode> = HashSet::new();
    let mut modifiers = Default::default();

    event_loop.run(move |event, _, control_flow| match event {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::CloseRequested => {
                *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                return;
            }
            event::WindowEvent::CursorMoved { position, .. } => {
                mouse.update(&position);
            }
            event::WindowEvent::MouseInput { state, button, .. } => {
                mouse_click_system(&mut world, &mut selected, button, state, modifiers, cursor_entity);
            }
            event::WindowEvent::MouseWheel { delta, ..} => {
                mouse_scroll_system(&mut world, delta, camera_entity);
            }
            event::WindowEvent::ModifiersChanged(state) => {
                modifiers = state;
            }
            event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                event::ElementState::Pressed => {
                    if let Some(keycode) = input.virtual_keycode {
                        pressed_keys.insert(keycode);
                    }
                }
                event::ElementState::Released => {
                    if let Some(keycode) = input.virtual_keycode {
                        pressed_keys.remove(&keycode);
                    }
                }
            },
            _ => {}
        },
        event::Event::MainEventsCleared => {
            rotate_system(&mut world);
            render_system(&display, &mut mesh_repo, &world, &shader, camera_entity);
            cursor_system(&mouse, &mut world, cursor_entity, camera_entity);
            select_system(&mut world, cursor_entity);
            camera_system(&mut world, &pressed_keys, camera_entity);
            movement_system(&mut world);
        }
        _ => {}
    });
}
