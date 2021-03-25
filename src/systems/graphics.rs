use super::prelude::*;
use crate::ecs::components::{Camera, MeshRenderer, Transform};
use crate::ecs::IntoQuery;
use crate::impls::graphics::matrix::CORRECTION_MATRIX;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use crate::math::{perspective, EuclideanSpace, Matrix4, Point3, SquareMatrix, Vector3};
use bytemuck::{Pod, Zeroable};
use log::warn;
use wgpu::ShaderStage;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: LambertPipeline,
}

impl GraphicsSystem {
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        let width = self.drivers.swap_chain_desc.width as f32;
        let height = self.drivers.swap_chain_desc.height as f32;
        width / height
    }

    fn compute_camera(&self, camera_entity: (&Transform, &Camera)) -> Matrix4<f32> {
        let trans = camera_entity.0;
        let cam = camera_entity.1;

        let projection_matrix =
            perspective(cam.fov, self.aspect_ratio(), cam.near_clip, cam.far_clip);

        let view_matrix = Matrix4::look_at_rh(
            Point3::from_vec(trans.position),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::unit_y(),
        );

        CORRECTION_MATRIX * projection_matrix * view_matrix
    }
}

impl SubSystem for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let drivers = Drivers::initialize(window, cfg);
        let lambert_pipeline = LambertPipeline::create(&drivers, cfg);

        Self {
            drivers,
            lambert_pipeline,
        }
    }

    fn tick(&mut self, world: &mut World) -> bool {
        let mut flag = true;
        let mut frame = self.drivers.begin_frame();
        {
            let mut pass = frame.create_pass();
            pass.set_pipeline(&self.lambert_pipeline);

            let camera = <(&Transform, &Camera)>::query().iter(world).next();
            let view_proj_matrix = if let Some(camera) = camera {
                self.compute_camera(camera)
            } else {
                flag = false;
                warn!("No camera found!");
                Matrix4::identity()
            };

            let mut query = <(&Transform, &MeshRenderer)>::query();
            for (transform, renderer) in query.iter(world) {
                let world_matrix = transform.compute_world();

                let push_constant_data = PushConstantData {
                    world_matrix,
                    view_proj_matrix,
                };

                pass.set_push_constans(
                    ShaderStage::VERTEX,
                    0,
                    bytemuck::bytes_of(&push_constant_data),
                );
                pass.set_bind_group(0, renderer.material.bind_group());
                pass.draw_indexed(&renderer.mesh);
            }
        }

        frame.end();
        flag
    }
}

#[derive(Copy, Clone)]
struct PushConstantData {
    pub world_matrix: Matrix4<f32>,
    pub view_proj_matrix: Matrix4<f32>,
}

unsafe impl Pod for PushConstantData {}
unsafe impl Zeroable for PushConstantData {}
