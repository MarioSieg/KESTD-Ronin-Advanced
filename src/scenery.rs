use super::components::{Camera, MeshRenderer, Transform};
use super::systems::SystemSupervisor;
use crate::resources::{
    material::{Material, MaterialProperties},
    ResourceManager,
};
use cgmath::*;
use legion::{Resources, World};
use std::path::PathBuf;

pub struct Scenery {
    pub world: World,
    pub resources: Resources,
}

impl Scenery {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            world: World::default(),
            resources: Resources::default(),
        })
    }

    pub fn default_preset(
        systems: &SystemSupervisor,
        resource_manager: &mut ResourceManager,
    ) -> Box<Scenery> {
        let mut scenery = Scenery::new();

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
                clamp_y: Deg(60.0),
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

        scenery
    }
}
