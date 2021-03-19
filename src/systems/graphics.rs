use super::prelude::*;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use crate::resources::mesh::Mesh;
use crate::resources::ResourceImporteur;
use std::path::PathBuf;
use wgpu::IndexFormat;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: ShaderPipeline,
}

impl System for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(window, is_power_safe_mode, use_vsync);

        let lambert_pipeline = pipelines::lambert::create(&drivers);

        Self {
            drivers,
            lambert_pipeline,
        }
    }

    fn tick(&mut self) -> bool {
        let mesh = Mesh::load(self, PathBuf::from("")).unwrap();

        let mut frame = self.drivers.begin_frame();
        {
            let mut pass = frame.create_pass();
            pass.set_pipeline(&self.lambert_pipeline.render_pipeline);
            pass.set_bind_group(0, &self.lambert_pipeline.bind_group, &[]);
            pass.set_index_buffer(mesh.index_buffer().slice(..), IndexFormat::Uint16);
            pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
            pass.draw_indexed(0..mesh.indices().len() as u32, 0, 0..1)
        }

        frame.end();
        true
    }
}
