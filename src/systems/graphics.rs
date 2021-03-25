use super::prelude::*;
use crate::ecs::components::MeshRenderer;
use crate::ecs::IntoQuery;
use crate::impls::graphics::matrix::CORRECTION_MATRIX;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};
use wgpu::ShaderStage;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: LambertPipeline,
}

impl SubSystem for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(
            window,
            is_power_safe_mode,
            use_vsync,
            cfg.graphics_config.msaa_mode,
        );

        let lambert_pipeline = LambertPipeline::create(&drivers, cfg);

        Self {
            drivers,
            lambert_pipeline,
        }
    }

    fn tick(&mut self, world: &mut World) -> bool {
        let mut frame = self.drivers.begin_frame();
        {
            let mut pass = frame.create_pass();
            pass.set_pipeline(&self.lambert_pipeline);

            let mx_projection = perspective(
                Deg(75.0),
                self.drivers.swap_chain_desc.width as f32
                    / self.drivers.swap_chain_desc.height as f32,
                1.0,
                10.0,
            );
            let mx_view = Matrix4::look_at_rh(
                Point3::new(1.5, 0.0, 5.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::unit_y(),
            );
            let transform = CORRECTION_MATRIX * mx_projection * mx_view;
            let transform_ref: &[f32; 16] = transform.as_ref();
            pass.set_push_constans(ShaderStage::VERTEX, 0, bytemuck::cast_slice(transform_ref));

            let mut query = <&MeshRenderer>::query();
            for renderer in query.iter(world) {
                pass.set_bind_group(0, renderer.material.bind_group());
                pass.draw_indexed(&renderer.mesh);
            }
        }

        frame.end();
        true
    }
}
