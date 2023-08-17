use glium::{Display, Surface};
use hecs::World;

use crate::{
    camera::Camera,
    mesh_repo::{MeshId, MeshRepo},
    transformation::Transformation, light::Light,
};

pub fn render_system(display: &Display, mesh_repo: &mut MeshRepo, world: &World) {
    let mut frame = display.draw();
    frame.clear_color(0.8, 0.7, 0.6, 1.0);

    let mut cameras = world.query::<(&Camera,)>();
    let (_, (camera,)) = cameras
        .iter()
        .last()
        .expect("Tried to render without a camera in scene");

    let mut lights = world.query::<(&Light,)>();
    let lights: Vec<&Light> = lights
        .iter()
        .map(|(_, (l,))| l)
        .collect();

    world.query::<(&MeshId, &Transformation)>().iter().for_each(
        |(_id, (mesh_id, transformation))| {
            let mesh = mesh_repo
                .get(mesh_id)
                .expect("MeshId does not correspond to any mesh in repo");

            mesh.render(
                &mut frame,
                transformation.model,
                camera.view,
                camera.projection,
                &lights
            );
        },
    );

    frame.finish().expect("Falied to draw to screen");
}
