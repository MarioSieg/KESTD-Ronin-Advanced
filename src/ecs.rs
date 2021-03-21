pub use specs::*;
pub use specs_derive as derive;

pub mod components {
    use super::{derive::*, Component, DenseVecStorage, World, WorldExt};
    use crate::resources::mesh::Mesh;
    use crate::resources::texture::Texture;
    use std::sync::Arc;

    #[derive(Component)]
    #[storage(DenseVecStorage)]
    pub struct MeshRenderer {
        pub texture: Arc<Texture>,
        pub mesh: Arc<Mesh>,
    }

    pub fn register_all(world: &mut World) {
        world.register::<MeshRenderer>();
    }
}
