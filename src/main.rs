extern crate glium;

use camera::Camera;
use glium::Display;
use hecs::World;
use light::Light;
use mesh_repo::MeshRepo;
use nalgebra_glm::{Mat4, Vec3};
use render::render_system;
use transformation::Transformation;

pub mod camera;
pub mod light;
pub mod math;
pub mod mesh;
pub mod mesh_repo;
pub mod render;
pub mod shader;
pub mod transformation;
pub mod vertex;
pub mod wavefront;

// Simple system that rotate its entities around the y-axis
struct Rotate {}
fn rotate_system(world: &mut World) {
    for (_id, (transformation, _rotate)) in world.query_mut::<(&mut Transformation, &Rotate)>() {
        transformation.model *= Mat4::new_rotation(Vec3::y() * 0.0001);
    }
}

fn initialize_glium() -> Display {
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Topdown");

    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).expect("Failed to create the display");
    display
}

fn main() {
    let display = initialize_glium();

    // Create the world
    let mut world = World::new();

    // Set up mesh repository and load shaders
    let mut mesh_repo = MeshRepo::new();
    let shader = shader::load(&display, "fragment.glsl".into(), "vertex.glsl".into());

    // Load an example mesh and add to mesh repo (cube)
    let mesh = mesh::Mesh::load(&display, "cube.obj".into(), &shader);
    let mesh_id = mesh_repo.insert(mesh);

    // Populate the world with a light, a camera and a loaded mesh object
    world.spawn((Camera::new(Vec3::new(0.0, 0.0, -10.0), 0.1, 30.0),));

    world.spawn((Light::new(
        Vec3::new(20.0, 20.0, -20.0),
        Vec3::new(0.7, 0.7, 0.7),
    ),));

    world.spawn((
        mesh_id,
        Transformation {
            model: Mat4::identity(),
        },
        Rotate {},
    ));


    loop {
        rotate_system(&mut world);
        render_system(&display, &mut mesh_repo, &world);
    }
}
