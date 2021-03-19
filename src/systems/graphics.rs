use super::System;
use crate::config::CoreConfig;
use crate::impls::graphics::Drivers;
use crate::load_shader;

pub struct GraphicsSystem {
    drivers: Drivers,
    render_pipeline: wgpu::RenderPipeline,
}

impl System for GraphicsSystem {
    type Args = glfw::Window;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(window, is_power_safe_mode, use_vsync);

        let lambert = drivers.load_shader_bundle(load_shader!("lambert"));

        let pipeline_layout =
            drivers
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            drivers
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    vertex: lambert.vertex_state(),
                    fragment: Some(lambert.fragment_state()),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                });

        Self {
            drivers,
            render_pipeline,
        }
    }

    fn tick(&mut self) -> bool {
        let mut frame = self.drivers.begin_frame();
        {
            let mut pass = frame.create_pass();
            pass.set_pipeline(&self.render_pipeline);
            pass.draw(0..3, 0..1);
        }

        frame.end();
        true
    }
}
