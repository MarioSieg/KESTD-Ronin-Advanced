use super::prelude::*;
use crate::ecs::components::MeshRenderer;
use crate::ecs::IntoQuery;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;

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
            pass.set_bind_group(
                0,
                &self.lambert_pipeline.shader_pipeline.internal_bind_group,
            );

            let mut query = <&MeshRenderer>::query();
            for renderer in query.iter(world) {
                pass.set_bind_group(1, renderer.material.bind_group());
                pass.draw_indexed(&renderer.mesh);
            }
        }

        frame.end();
        true
    }
}
