pub mod components {
    use crate::resources::{material::Material, mesh::Mesh};
    use std::sync::Arc;

    pub struct MeshRenderer {
        pub mesh: Arc<Mesh>,
        pub material: Arc<Material>,
    }
}
