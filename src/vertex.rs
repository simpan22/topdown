use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

#[derive(Clone, Copy)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);
