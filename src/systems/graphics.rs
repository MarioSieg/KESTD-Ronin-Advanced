use super::System;
use crate::config::CoreConfig;
use crate::impls::graphics::Drivers;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl System<glfw::Window> for GraphicsSystem {
    fn initialize(cfg: &mut CoreConfig, window: &glfw::Window) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(window, is_power_safe_mode, use_vsync);

        let vertex_shader = drivers.device.create_shader_module(&wgpu::include_spirv!(
            "../../db/shaders/fixed_pipelines/lambert/final/shader.vert.spv"
        ));
        let fragment_shader = drivers.device.create_shader_module(&wgpu::include_spirv!(
            "../../db/shaders/fixed_pipelines/lambert/final/shader.frag.spv"
        ));

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
                    vertex: wgpu::VertexState {
                        module: &vertex_shader,
                        entry_point: "main",
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &fragment_shader,
                        entry_point: "main",
                        targets: &[drivers.swap_chain_format.into()],
                    }),
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
