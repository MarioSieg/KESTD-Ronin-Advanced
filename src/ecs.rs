use crate::math::{Array, Deg, Matrix4, Quaternion, Vector3, Zero};
use crate::resources::{
    material::{Material, MaterialProperties},
    mesh::Mesh,
    texture::Texture,
    ResourceImporteur,
};
use std::sync::Arc;

pub use legion::*;

pub mod components {
    use super::*;

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
    }
}

use crate::systems::SystemSupervisor;
use components::*;

pub fn initialize_default_world(systems: &SystemSupervisor, world: &mut World) {
    use std::path::PathBuf;

    let cube1 = (
        Transform {
            position: Vector3::new(3.0, 0.0, 0.0),
            rotation: Quaternion::zero(),
            scale: Vector3::from_value(1.0),
        },
        MeshRenderer {
            mesh: Mesh::load(&systems.graphics, PathBuf::from("db/meshes/cube.obj")),
            material: Material::load(
                &systems.graphics,
                MaterialProperties::Lambert {
                    albedo: Texture::load(
                        &systems.graphics,
                        PathBuf::from("db/textures/metal.png"),
                    ),
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
            mesh: Mesh::load(&systems.graphics, PathBuf::from("db/meshes/cube.obj")),
            material: Material::load(
                &systems.graphics,
                MaterialProperties::Lambert {
                    albedo: Texture::load(&systems.graphics, PathBuf::from("db/textures/wood.png")),
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
        },
    );

    world.push(cube1);
    world.push(cube2);
    world.push(camera);
}
