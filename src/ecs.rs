pub mod components {
    use crate::resources::{mesh::Mesh, texture::Texture};
    use std::sync::Arc;

    pub struct MeshRenderer {
        pub mesh: Arc<Mesh>,
        pub texture: Arc<Texture>,
    }
}
