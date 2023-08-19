extern crate glium;

use camera::Camera;
use glium::glutin::event;
use glium::{glutin::event_loop::EventLoop, Display};
use hecs::World;
use light::Light;
use mesh_repo::MeshRepo;
use mouse::Mouse;
use nalgebra_glm::{Mat4, Vec3};
use render::render_system;
use transformation::Transformation;

pub mod camera;
pub mod light;
pub mod math;
pub mod mesh;
pub mod mesh_repo;
pub mod mouse;
pub mod render;
pub mod shader;
pub mod transformation;
pub mod vertex;
pub mod wavefront;

static WIDTH: i32 = 1024;
static HEIGHT: i32 = 768;

// Simple system that rotate its entities around the y-axis
struct Rotate {}
fn rotate_system(world: &mut World) {
    for (_id, (transformation, _rotate)) in world.query_mut::<(&mut Transformation, &Rotate)>() {
        transformation.model *= Mat4::new_rotation(Vec3::y() * 0.0001);
    }
}

fn initialize_glium(w: i32, h: i32) -> (Display, EventLoop<()>) {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(w, h))
        .with_title("Topdown");

    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).expect("Failed to create the display");

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

    // Load an example mesh and add to mesh repo (cube)
    let mesh = mesh::Mesh::load(&display, "cube.obj".into());
    let mesh_id = mesh_repo.insert(mesh);

    // Populate the world with a light, a camera and a loaded mesh object
    world.spawn((Camera::new(
        Vec3::new(0.0, 7.0, 10.0), // eye
        Vec3::new(0.0, 0.0, 0.0),  // center
        0.1,
        30.0,
    ),));

    world.spawn((
        Light::new(
            Vec3::new(0.0, 0.0, -3.0), // Pos
            Vec3::new(0.7, 0.7, 0.7), // Color
        ),
        mesh_id.clone(),
        Transformation {
            model: Mat4::new_translation(&Vec3::new(0.0, 0.0, -3.0)) * Mat4::new_scaling(0.2),
        },
    ));

    world.spawn((
        mesh_id.clone(),
        Transformation {
            model: Mat4::new_translation(&Vec3::new(0.0, 0.0, 0.0)),
        },
        Rotate {},
    ));

    event_loop.run(move |event, _, control_flow| {
        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                event::WindowEvent::CursorMoved { position, .. } => {
                    mouse.update(&position);
                }
                _ => {}
            },
            event::Event::MainEventsCleared => {
                // Update systems
                rotate_system(&mut world);
                render_system(&display, &mut mesh_repo, &world, &shader);
            }
            _ => {}
        }
    });
}
