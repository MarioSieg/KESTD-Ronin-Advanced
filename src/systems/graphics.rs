use super::System;
use crate::config::CoreConfig;
use crate::impls::graphics::{Drivers, ShaderPipeline, ShaderPipelineDescriptor};
use crate::load_shader;

pub struct GraphicsSystem {
    drivers: Drivers,
    lambert_pipeline: ShaderPipeline,
}

impl System for GraphicsSystem {
    type Args = glfw::Window;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(window, is_power_safe_mode, use_vsync);

        let lambert_pipeline = drivers.create_shader_bundle(ShaderPipelineDescriptor::new_simple(
            load_shader!("lambert"),
        ));

        Self {
            drivers,
            lambert_pipeline,
        }
    }

    fn tick(&mut self) -> bool {
        let mut frame = self.drivers.begin_frame();
        {
            let mut pass = frame.create_pass();
            pass.set_pipeline(&self.lambert_pipeline.render_pipeline);
            pass.draw(0..3, 0..1);
        }

        frame.end();
        true
    }
}
