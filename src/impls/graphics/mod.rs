pub mod matrix;
pub mod pass;
pub mod pipeline;
pub mod pipelines;
pub mod prelude;

use crate::config::MsaaMode;
use crate::impls::graphics::prelude::Pipeline;
use log::info;
use pass::Pass;
use pipeline::{ShaderPipeline, ShaderPipelineDescriptor};
use wgpu::*;

pub struct Drivers {
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub swap_chain: SwapChain,
    pub swap_chain_desc: SwapChainDescriptor,
    pub swap_chain_format: TextureFormat,
    pub frame_buffer: TextureView,
    pub msaa_samples: MsaaMode,
}

pub struct Frame<'a> {
    pub view: SwapChainTexture,
    pub encoder: CommandEncoder,
    pub queue: &'a Queue,
    pub frame_buf: &'a TextureView,
    samples: MsaaMode,
}

impl<'a> Frame<'a> {
    pub fn create_pass(&mut self) -> Pass {
        let ops = Operations {
            load: LoadOp::Clear(Color::WHITE),
            store: true,
        };
        let color_attachment = if self.samples == MsaaMode::Off {
            RenderPassColorAttachmentDescriptor {
                attachment: &self.view.view,
                resolve_target: None,
                ops,
            }
        } else {
            RenderPassColorAttachmentDescriptor {
                attachment: self.frame_buf,
                resolve_target: Some(&self.view.view),
                ops,
            }
        };
        let render_pass = self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[color_attachment],
            depth_stencil_attachment: None,
        });
        Pass(render_pass)
    }

    pub fn end(self) {
        self.queue.submit(Some(self.encoder.finish()));
    }
}

impl Drivers {
    pub fn create_shader_pipeline<T: Pipeline>(
        &self,
        desc: ShaderPipelineDescriptor,
    ) -> ShaderPipeline {
        ShaderPipeline::create_shader_bundle::<T>(self, desc)
    }

    pub fn begin_frame(&self) -> Frame {
        let view = self
            .swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture!")
            .output;
        let encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        Frame {
            view,
            encoder,
            queue: &self.queue,
            frame_buf: &self.frame_buffer,
            samples: self.msaa_samples,
        }
    }

    pub fn initialize(
        window: &glfw::Window,
        power_safe_mode: bool,
        vsync: bool,
        msaa_samples: MsaaMode,
    ) -> Self {
        let instance = Instance::new(BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let (adapter, device, queue) = futures::executor::block_on(Self::create_async_resources(
            &instance,
            &surface,
            power_safe_mode,
        ));

        let info = adapter.get_info();

        info!("GPU: {}", info.name);
        info!("API: {:?}", info.backend);
        info!("Type: {:?}", info.device_type);

        let swap_chain_format = adapter.get_swap_chain_preferred_format(&surface);

        let swap_chain_desc = SwapChainDescriptor {
            usage: TextureUsage::RENDER_ATTACHMENT | TextureUsage::COPY_SRC,
            format: swap_chain_format,
            width: window.get_framebuffer_size().0 as _,
            height: window.get_framebuffer_size().1 as _,
            present_mode: if vsync {
                PresentMode::Fifo
            } else {
                PresentMode::Mailbox
            },
        };

        info!("Swapchain descriptor:\n{:#?}", swap_chain_desc);
        info!("MSAA samples: {:?}", msaa_samples);

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let frame_buffer =
            Self::create_multisampled_framebuffer(&device, &swap_chain_desc, msaa_samples as u32);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            swap_chain,
            swap_chain_desc,
            swap_chain_format,
            frame_buffer,
            msaa_samples,
        }
    }

    async fn create_async_resources(
        instance: &Instance,
        surface: &Surface,
        low_power_mode: bool,
    ) -> (Adapter, Device, Queue) {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: if low_power_mode {
                    PowerPreference::LowPower
                } else {
                    PowerPreference::HighPerformance
                },
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find GPU adapter!");
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::default(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device and queue!");
        (adapter, device, queue)
    }

    fn create_multisampled_framebuffer(
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        sample_count: u32,
    ) -> TextureView {
        let multisampled_texture_extent = Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth: 1,
        };
        let multisampled_frame_descriptor = &TextureDescriptor {
            size: multisampled_texture_extent,
            mip_level_count: 1,
            sample_count,
            dimension: TextureDimension::D2,
            format: sc_desc.format,
            usage: TextureUsage::RENDER_ATTACHMENT,
            label: None,
        };

        device
            .create_texture(multisampled_frame_descriptor)
            .create_view(&TextureViewDescriptor::default())
    }
}
