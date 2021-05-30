use crate::math::{Array, Deg, Quaternion, Vector3, Zero};
use crate::resources::{
    material::{Material, MaterialProperties},
    ResourceManager,
};
use std::path::PathBuf;

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
    use crate::impls::platform::prelude::{KEY_COUNT, MOUSE_BUTTON_COUNT};

    #[derive(Default, Copy, Clone, Debug)]
    pub struct CursorPos(pub f32, pub f32);

    #[derive(Clone, Debug)]
    pub struct KeyInputStateCollection([bool; KEY_COUNT]);

    impl KeyInputStateCollection {
        #[inline]
        pub fn reset(&mut self) {
            self.0.fill(false)
        }

        #[inline]
        pub fn push(&mut self, key: Key) {
            self.0[key as usize] = true
        }

        #[inline]
        pub fn pop(&mut self, key: Key) {
            self.0[key as usize] = false
        }

        #[inline]
        pub fn is_key_pressed(&self, key: Key) -> bool {
            self.0[key as usize]
        }
    }

    impl std::default::Default for KeyInputStateCollection {
        fn default() -> Self {
            Self([false; KEY_COUNT])
        }
    }

    pub struct MouseInputStateCollection([bool; MOUSE_BUTTON_COUNT]);

    impl MouseInputStateCollection {
        #[inline]
        pub fn reset(&mut self) {
            self.0.fill(false)
        }

        #[inline]
        pub fn push(&mut self, key: MouseButton) {
            self.0[key as usize] = true
        }

        #[inline]
        pub fn pop(&mut self, key: MouseButton) {
            self.0[key as usize] = false
        }

        #[inline]
        pub fn is_key_pressed(&self, key: MouseButton) -> bool {
            self.0[key as usize]
        }
    }

    impl std::default::Default for MouseInputStateCollection {
        fn default() -> Self {
            Self([false; MOUSE_BUTTON_COUNT])
        }
    }
}

use super::components::{Camera, MeshRenderer, Transform};
use super::systems::SystemSupervisor;

pub fn initialize_default_scenery(
    systems: &SystemSupervisor,
    scenery: &mut Scenery,
    resource_manager: &mut ResourceManager,
) {
    let camera = (
        Transform {
            position: Vector3::new(0.0, 2.0, 0.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
        },
        Camera {
            fov: Deg(75.0),
            near_clip: 0.1,
            far_clip: 100.0,
            clamp_y: 1.2,
            ..Default::default()
        },
    );

    scenery.world.push(camera);

    let mut cube = (
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
        },
        MeshRenderer {
            mesh: resource_manager
                .mesh_cache
                .import(&systems.graphics, PathBuf::from("db/meshes/cube.obj")),
            material: Material::load(
                &systems.graphics,
                MaterialProperties::Lambert {
                    albedo: resource_manager
                        .texture_cache
                        .import(&systems.graphics, PathBuf::from("db/textures/grid.png")),
                },
            ),
        },
    );

    for i in 0..4 {
        for j in 0..4 {
            cube.0.position.x = j as f32;
            cube.0.position.z = i as f32;
            cube.0.scale = Vector3::from_value(0.25);
            scenery.world.push(cube.clone());
        }
    }
}
