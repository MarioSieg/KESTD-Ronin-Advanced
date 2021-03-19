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
        let frame = self
            .drivers
            .swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture!")
            .output;
        let mut encoder = self
            .drivers
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            pass.set_pipeline(&self.render_pipeline);
            pass.draw(0..3, 0..1);
        }

        self.drivers.queue.submit(Some(encoder.finish()));
        true
    }
}
