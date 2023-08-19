use std::collections::HashMap;

use crate::mesh::Mesh;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct MeshId(u32);

pub struct MeshRepo {
    entries: HashMap<MeshId, Mesh>,
    n_meshes: u32,
}

impl MeshRepo {
    pub fn new() -> Self {
        MeshRepo {
            entries: HashMap::new(),
            n_meshes: 0,
        }
    }

    pub fn insert(&mut self, mesh: Mesh) -> MeshId {
        let id = MeshId(self.n_meshes);
        self.entries.insert(id.clone(), mesh);
        self.n_meshes += 1;
        id
    }

    pub fn get(&mut self, id: &MeshId) -> Option<&Mesh> {
        self.entries.get(id)
    }
}
