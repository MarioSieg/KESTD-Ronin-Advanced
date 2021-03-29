use crate::math::{Array, Deg, Matrix4, Quaternion, Vector3, Zero};
use crate::resources::{
    material::{Material, MaterialProperties},
    mesh::Mesh,
    ResourceManager,
};
use std::sync::Arc;

pub use legion::*;

pub struct Scenery {
    pub world: World,
    pub resources: Resources,
}

impl Scenery {
    pub fn new() -> Self {
        Self {
            world: World::default(),
            resources: Resources::default(),
        }
    }
}

impl std::default::Default for Scenery {
    fn default() -> Self {
        Self::new()
    }
}

pub mod resources {
    pub use crate::impls::platform::prelude::{Action, Key, Modifiers, MouseButton};
    use smallvec::SmallVec;

    #[derive(Default, Copy, Clone, Debug)]
    pub struct CursorPos(pub f32, pub f32);

    #[derive(Clone, Debug)]
    pub struct KeyInput {
        pub key: Key,
        pub action: Action,
        pub modifier: Modifiers,
    }

    #[derive(Clone, Debug)]
    pub struct KeyInputQueue(pub SmallVec<[KeyInput; 64]>);

    #[derive(Clone, Debug)]
    pub struct MouseInput {
        pub button: MouseButton,
        pub action: Action,
        pub modifier: Modifiers,
    }

    pub struct MouseInputQueue(pub SmallVec<[MouseInput; 8]>);
}

pub mod components {
    use super::*;
    use crate::math::Vector2;

    #[derive(Clone)]
    pub struct Transform {
        pub position: Vector3<f32>,
        pub rotation: Quaternion<f32>,
        pub scale: Vector3<f32>,
    }

    impl Transform {
        #[inline]
        pub fn calculate_matrix(&self) -> Matrix4<f32> {
            Matrix4::from_translation(self.position)
                * Matrix4::from(self.rotation)
                * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
        }
    }

    #[derive(Clone)]
    pub struct MeshRenderer {
        pub mesh: Arc<Mesh>,
        pub material: Arc<Material>,
    }

    #[derive(Clone)]
    pub struct Camera {
        pub fov: Deg<f32>,
        pub near_clip: f32,
        pub far_clip: f32,
        pub prev: Vector2<f32>,
        pub angles: Vector2<f32>,
    }

    impl Default for Camera {
        fn default() -> Self {
            Self {
                fov: Deg(75.0),
                near_clip: 0.1,
                far_clip: 100.0,
                prev: Vector2::zero(),
                angles: Vector2::zero(),
            }
        }
    }
}

use crate::systems::SystemSupervisor;
use components::*;

pub fn initialize_default_scenery(
    systems: &SystemSupervisor,
    scenery: &mut Scenery,
    resource_manager: &mut ResourceManager,
) {
    let cube1 = (
        Transform {
            position: Vector3::new(3.0, 0.0, 0.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
        },
        MeshRenderer {
            mesh: resource_manager
                .mesh_cache
                .load_imm(&systems.graphics, "db/meshes/cube.obj"),
            material: Material::load(
                &systems.graphics,
                MaterialProperties::Lambert {
                    albedo: resource_manager
                        .texture_cache
                        .load_imm(&systems.graphics, "db/textures/metal.png"),
                },
            ),
        },
    );

    let cube2 = (
        Transform {
            position: Vector3::new(0.0, 1.0, 0.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
        },
        MeshRenderer {
            mesh: resource_manager
                .mesh_cache
                .load_imm(&systems.graphics, "db/meshes/cube.obj"),
            material: Material::load(
                &systems.graphics,
                MaterialProperties::Lambert {
                    albedo: resource_manager
                        .texture_cache
                        .load_imm(&systems.graphics, "db/textures/wood.png"),
                },
            ),
        },
    );

    let camera = (
        Transform {
            position: Vector3::new(0.0, 0.0, 5.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
        },
        Camera {
            fov: Deg(75.0),
            near_clip: 0.1,
            far_clip: 100.0,
            ..Default::default()
        },
    );

    scenery.world.push(cube1);
    scenery.world.push(cube2);
    scenery.world.push(camera);
}
