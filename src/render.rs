use glium::{
    index::NoIndices, uniform, BackfaceCullingMode, DepthTest, Display, DrawParameters, Program,
    Surface,
};
use hecs::World;

use crate::{
    camera::Camera,
    light::Light,
    mesh_repo::{MeshId, MeshRepo},
    transformation::Transformation,
};

pub fn render_system(display: &Display, mesh_repo: &mut MeshRepo, world: &World, shader: &Program) {
    let mut frame = display.draw();
    frame.clear(None, Some((0.8, 0.7, 0.6, 1.0)), true, Some(1.0), Some(0));

    let mut cameras = world.query::<(&Camera,)>();
    let (_, (camera,)) = cameras
        .iter()
        .last()
        .expect("Tried to render without a camera in scene");

    let mut lights = world.query::<(&Light,)>();
    let lights: Vec<&Light> = lights.iter().map(|(_, (l,))| l).collect();

    world.query::<(&MeshId, &Transformation)>().iter().for_each(
        |(_id, (mesh_id, transformation))| {
            let mesh = mesh_repo
                .get(mesh_id)
                .expect("MeshId does not correspond to any mesh in repo");

            // Uniforms need to be converted to gliums plain format
            let model: [[f32; 4]; 4] = transformation.model.into();
            let view: [[f32; 4]; 4] = camera.view.into();
            let projection: [[f32; 4]; 4] = camera.projection.into();
            let light_pos: [f32; 3] = lights[0].position.into();

            frame
                .draw(
                    &mesh.vertex_buffer,
                    &NoIndices(glium::index::PrimitiveType::TrianglesList),
                    shader,
                    &uniform! {model: model, view: view, projection: projection, light_pos: light_pos},

                    &DrawParameters {
                        backface_culling: BackfaceCullingMode::CullClockwise,
                        depth: glium::Depth { 
                            test: DepthTest::IfLess, 
                            write: true, 
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )
                .expect("Failed to render");
        },
    );

    frame.finish().expect("Falied to draw to screen");
}
