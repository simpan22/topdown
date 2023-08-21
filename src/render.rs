use glium::{
    index::{NoIndices, PrimitiveType},
    texture::SrgbTexture2d,
    uniform, BackfaceCullingMode, DepthTest, Display, DrawParameters, Frame, Program, Surface,
    VertexBuffer,
};
use hecs::{Entity, World};
use nalgebra_glm::{vec3, Mat4, Vec3};

use crate::{
    camera::Camera,
    light::Light,
    mesh_repo::{MeshId, MeshRepo},
    mouse::{create_cursor_vb, Cursor},
    selectable::Selectable,
    transformation::Transformation,
    vertex::Vertex,
};

pub fn render_system(
    display: &Display,
    mesh_repo: &mut MeshRepo,
    world: &World,
    shader: &Program,
    camera_entity: Entity,
) {
    let mut frame = display.draw();
    frame.clear(None, Some((0.8, 0.7, 0.6, 1.0)), true, Some(1.0), Some(0));

    let camera = world.get::<&Camera>(camera_entity).unwrap();

    let mut lights = world.query::<(&Light,)>();
    let lights: Vec<&Light> = lights.iter().map(|(_, (l,))| l).collect();

    // Render meshes
    world.query::<(&MeshId, &Transformation)>().iter().for_each(
        |(_id, (mesh_id, transformation))| {
            let mesh = mesh_repo
                .get(mesh_id)
                .expect("MeshId does not correspond to any mesh in repo");

            render_vertex_buffer(
                &mut frame,
                &mesh.vertex_buffer,
                glium::index::PrimitiveType::TrianglesList,
                &shader,
                transformation.model(),
                &camera,
                lights[0],
                mesh.color,
                &mesh.texture,
            );
        },
    );

    // Render bounding circles around selectables
    world
        .query::<(&Transformation, &Selectable)>()
        .iter()
        .for_each(|(_id, (transformation, selectable))| {
            if selectable.hover || selectable.selected {
                let bc_vertex_buffer = VertexBuffer::new(
                    display,
                    &selectable
                        .bounding_circle
                        .triangle_strip(24, 0.1 / transformation.scale),
                )
                .unwrap();

                let bc_color = if selectable.hover {
                    vec3(0.1, 0.1, 0.7)
                } else {
                    vec3(0.1, 0.1, 0.1)
                };

                let bc_model =
                    Mat4::new_translation(&vec3(transformation.pos.x, 0.1, transformation.pos.z))
                        * Mat4::new_scaling(transformation.scale);

                render_vertex_buffer(
                    &mut frame,
                    &bc_vertex_buffer,
                    glium::index::PrimitiveType::TriangleStrip,
                    &shader,
                    bc_model,
                    &camera,
                    lights[0],
                    bc_color,
                    // TODO: Don't recreate and drop textures every render
                    &SrgbTexture2d::empty(display, 1, 1).unwrap(),
                );
            }
        });

    frame.clear_depth(1.0);
    // Draw cursor
    world
        .query::<(&Cursor,)>()
        .iter()
        .for_each(|(_, (cursor,))| {
            // Model matrix for debug point is just a translation to the specified point
            let model = Mat4::new_translation(&cursor.position);
            let object_color = Vec3::new(1.0, 0.0, 0.0);

            render_vertex_buffer(
                &mut frame,
                &create_cursor_vb(display),
                glium::index::PrimitiveType::LinesList,
                &shader,
                model,
                &camera,
                lights[0],
                object_color,
                // TODO: Don't recreate and drop textures every render
                &SrgbTexture2d::empty(display, 1, 1).unwrap(),
            );
        });

    frame.finish().expect("Falied to draw to screen");
}

fn render_vertex_buffer(
    frame: &mut Frame,
    vertices: &VertexBuffer<Vertex>,
    primitive_type: PrimitiveType,
    shader: &Program,
    model: Mat4,
    camera: &Camera,
    light: &Light,
    object_color: Vec3,
    texture: &SrgbTexture2d,
) {
    let model: [[f32; 4]; 4] = model.into();
    let view: [[f32; 4]; 4] = camera.view.into();
    let projection: [[f32; 4]; 4] = camera.projection.into();
    let light_pos: [f32; 3] = light.position.into();
    let light_color: [f32; 3] = light.color.into();
    let object_color: [f32; 3] = object_color.into();

    let texture = texture
        .sampled()
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
    let uniforms = uniform! {
        model: model,
        view: view,
        projection: projection,
        light_pos: light_pos,
        light_color: light_color,
        object_color: object_color,
        texture_sampler: texture
    };
    frame
        .draw(
            vertices,
            &NoIndices(primitive_type),
            &shader,
            &uniforms,
            &DrawParameters {
                backface_culling: BackfaceCullingMode::CullingDisabled,
                depth: glium::Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                line_width: Some(3.0),
                ..Default::default()
            },
        )
        .expect("Failed to render");
}
