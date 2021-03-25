use crate::resources::{
    material::{Material, MaterialProperties},
    mesh::Mesh,
    texture::Texture,
    ResourceImporteur,
};
use crate::systems::SystemSupervisor;
pub use legion::*;
use std::path::PathBuf;

pub mod components {
    use crate::resources::{material::Material, mesh::Mesh};
    use std::sync::Arc;

    #[derive(Clone)]
    pub struct MeshRenderer {
        pub mesh: Arc<Mesh>,
        pub material: Arc<Material>,
    }
}

use components::*;

pub fn initialize_default_world(systems: &SystemSupervisor, world: &mut World) {
    let renderer = MeshRenderer {
        mesh: Mesh::load(&systems.graphics, PathBuf::from("db/meshes/cube.obj")),
        material: Material::load(
            &systems.graphics,
            MaterialProperties::Lambert {
                albedo: Texture::load(&systems.graphics, PathBuf::from("db/textures/grid.png")),
            },
        ),
    };

    let _entity = world.push((renderer,));
}
