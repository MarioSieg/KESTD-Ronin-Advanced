use super::System;
use crate::config::CoreConfig;
use log::info;
use std::ops::BitOr;

pub struct GraphicsSystem {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub render_pipeline: wgpu::RenderPipeline,
    pub swap_chain: wgpu::SwapChain,
}

impl GraphicsSystem {
    async fn create_async_resources(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
        power_mode: bool,
    ) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue) {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: if power_mode {
                    wgpu::PowerPreference::LowPower
                } else {
                    wgpu::PowerPreference::HighPerformance
                },
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find GPU adapter!");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device and queue!");
        (adapter, device, queue)
    }
}

impl System<glfw::Window> for GraphicsSystem {
    fn initialize(cfg: &mut CoreConfig, window: &glfw::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let (adapter, device, queue) = futures::executor::block_on(Self::create_async_resources(
            &instance,
            &surface,
            cfg.application_config.power_safe_mode,
        ));

        let vertex_shader = device.create_shader_module(&wgpu::include_spirv!(
            "../../db/shaders/fixed_pipelines/lambert/final/shader.vert.spv"
        ));
        let fragment_shader = device.create_shader_module(&wgpu::include_spirv!(
            "../../db/shaders/fixed_pipelines/lambert/final/shader.frag.spv"
        ));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                targets: &[swapchain_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT.bitor(wgpu::TextureUsage::COPY_SRC),
            format: swapchain_format,
            width: window.get_framebuffer_size().0 as _,
            height: window.get_framebuffer_size().1 as _,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let info = adapter.get_info();
        info!("GPU: {}", info.name);
        info!("API: {:?}", info.backend);
        info!("Type: {:?}", info.device_type);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            render_pipeline,
            swap_chain,
        }
    }

    fn tick(&mut self) -> bool {
        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture!")
            .output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            rpass.set_pipeline(&self.render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        true
    }
}

#[rustfmt::skip]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
