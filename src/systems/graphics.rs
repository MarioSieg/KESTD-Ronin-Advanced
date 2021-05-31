use super::prelude::*;
use crate::components::{Camera, MeshRenderer, Transform};
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use crate::scenery_resources::{KeyInputStateCollection, MouseInputStateCollection};
use cgmath::{Matrix4, SquareMatrix};
use legion::IntoQuery;
use log::warn;
use wgpu::ShaderStage;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: lambert::LambertPipeline,
}

impl GraphicsSystem {
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        let width = self.drivers.swap_chain_desc.width as f32;
        let height = self.drivers.swap_chain_desc.height as f32;
        width / height
    }
}

impl SubSystem for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let mut drivers = Drivers::initialize(window, cfg);
        let lambert_pipeline = lambert::LambertPipeline::create(&mut drivers, cfg);

        Self {
            drivers,
            lambert_pipeline,
        }
    }

    fn tick(&mut self, scenery: &mut Scenery) -> bool {
        let mut flag = true;
        let mut frame = self.drivers.begin_frame();
        {
            let camera = <(&mut Transform, &mut Camera)>::query()
                .iter_mut(&mut scenery.world)
                .next();
            let view_proj_matrix = if let Some(camera) = camera {
                let cursor_pos = *scenery.resources.get_mut_or_default();
                let key_queue = scenery.resources.get::<KeyInputStateCollection>().unwrap();
                let mouse_queue = scenery
                    .resources
                    .get::<MouseInputStateCollection>()
                    .unwrap();
                camera::compute_camera(
                    self.aspect_ratio(),
                    camera,
                    cursor_pos,
                    &*key_queue,
                    &*mouse_queue,
                )
            } else {
                warn!("No camera found!");
                flag = false;
                Matrix4::identity()
            };

            let mut pass = frame.create_pass(true);
            pass.set_pipeline(&self.lambert_pipeline);

            let mut render_query = <(&Transform, &MeshRenderer)>::query();
            render_query.for_each(&scenery.world, |(transform, renderer)| {
                let world_matrix = transform.calculate_matrix();
                let push_constant_data = lambert::PushConstantData {
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
            });
        }

        frame.end();
        flag
    }
}
