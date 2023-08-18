use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    // pub texture_coord: [f32; 2]
}

implement_vertex!(Vertex, position, normal, /*texture_coord*/ );

