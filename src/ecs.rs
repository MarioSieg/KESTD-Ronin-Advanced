use crate::math::{Array, Deg, Matrix4, Quaternion, Vector3, Zero, SquareMatrix};
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
    pub struct KeyInputQueue(pub SmallVec<[Key; 64]>);

    impl KeyInputQueue {
        pub fn is_key_pressed(&self, key: Key) -> bool {
            self.0.contains(&key)
        }
    }

    pub struct MouseInputQueue(pub SmallVec<[MouseButton; 8]>);

    impl MouseInputQueue {
        pub fn is_key_pressed(&self, button: MouseButton) -> bool {
            self.0.contains(&button)
        }
    }
}

pub mod components {
    use super::*;
    use crate::math::Vector2;

    #[derive(Clone)]
    pub struct Transform {
        pub position: Vector3<f32>,
        pub rotation: Quaternion<f32>,
        pub scale: Vector3<f32>,
        pub matrix: Matrix4<f32>,
    }

    impl Transform {
        #[inline]
        pub fn calculate_matrix(&mut self) {
            self.matrix = Matrix4::from_translation(self.position)
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
        pub clamp_y: f32,
        pub smoothness: f32,
        pub speed: f32,
        pub prev: Vector2<f32>,
        pub angles: Vector2<f32>,
        pub smooth_angles: Vector2<f32>,
        pub forward: Vector3<f32>,
    }

    impl Default for Camera {
        fn default() -> Self {
            Self {
                fov: Deg(75.0),
                near_clip: 0.1,
                far_clip: 100.0,
                clamp_y: 80.0,
                smoothness: 1.5,
                speed: 0.01,
                prev: Vector2::zero(),
                angles: Vector2::zero(),
                smooth_angles: Vector2::zero(),
                forward: Vector3::zero(),
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
    let camera = (
        Transform {
            position: Vector3::new(0.0, 0.0, 5.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
            matrix: Matrix4::identity(),
        },
        Camera {
            fov: Deg(75.0),
            near_clip: 0.1,
            far_clip: 100.0,
            ..Default::default()
        },
    );

    scenery.world.push(camera);

    let mut tree = (
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
            matrix: Matrix4::identity(),
        },
        MeshRenderer {
            mesh: resource_manager
                .mesh_cache
                .load_imm(&systems.graphics, "db/meshes/tree.obj"),
            material: Material::load(
                &systems.graphics,
                MaterialProperties::Lambert {
                    albedo: resource_manager
                        .texture_cache
                        .load_imm(&systems.graphics, "db/textures/tree.png"),
                },
            ),
        },
    );

    for i in 0..64 {
        for j in 0..64 {
            tree.0.position.x = j as f32;
            tree.0.position.z = i as f32;
            tree.0.scale = Vector3::from_value(0.25);
            scenery.world.push(tree.clone());
        }
    }
}
