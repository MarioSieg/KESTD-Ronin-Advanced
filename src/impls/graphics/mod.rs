pub mod matrix;
pub mod pass;
pub mod pipeline;
pub mod pipelines;
pub mod prelude;

use crate::config::{CoreConfig, GraphicsApi, GraphicsConfig, MsaaMode};
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
    pub depth_texture: TextureView,
    pub msaa_samples: MsaaMode,
    blit_shader: (ShaderModule, ShaderModule),
}

pub struct Frame<'a> {
    pub view: SwapChainTexture,
    pub encoder: CommandEncoder,
    pub queue: &'a Queue,
    pub frame_buf: &'a TextureView,
    pub depth_stencil: &'a TextureView,
    samples: MsaaMode,
}

impl<'a> Frame<'a> {
    pub fn create_pass(&mut self, use_depth_stencil: bool) -> Pass {
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
            depth_stencil_attachment: if use_depth_stencil {
                Some(RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                })
            } else {
                None
            },
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
            depth_stencil: &self.depth_texture,
            samples: self.msaa_samples,
        }
    }

    pub fn initialize(window: &glfw::Window, config: &CoreConfig) -> Self {
        let backend_bit = match config.graphics_config.backend_api {
            GraphicsApi::Auto => BackendBit::PRIMARY,
            GraphicsApi::Direct3D11 => BackendBit::DX11,
            GraphicsApi::Direct3D12 => BackendBit::DX12,
            GraphicsApi::OpenGl => BackendBit::GL,
            GraphicsApi::Vulkan => BackendBit::VULKAN,
            GraphicsApi::WebGpu => BackendBit::BROWSER_WEBGPU,
        };
        let instance = Instance::new(backend_bit);
        let surface = unsafe { instance.create_surface(window) };
        let (adapter, device, queue) = futures::executor::block_on(Self::create_async_resources(
            &instance,
            &surface,
            config.application_config.power_safe_mode,
            &config.graphics_config,
        ));

        let info = adapter.get_info();

        info!("GPU: {}", info.name);
        info!("API: {:?}", info.backend);
        info!("Type: {:?}", info.device_type);
        let swap_chain_format = adapter.get_swap_chain_preferred_format(&surface);
        info!(
            "Texture format features: {:?}",
            adapter.get_texture_format_features(swap_chain_format)
        );

        let swap_chain_desc = SwapChainDescriptor {
            usage: TextureUsage::RENDER_ATTACHMENT | TextureUsage::COPY_SRC,
            format: swap_chain_format,
            width: window.get_framebuffer_size().0 as _,
            height: window.get_framebuffer_size().1 as _,
            present_mode: if config.display_config.vsync {
                PresentMode::Fifo
            } else {
                PresentMode::Mailbox
            },
        };

        info!("Swap chain usage: {:#?}", swap_chain_desc.usage);
        info!("Swap chain format: {:#?}", swap_chain_desc.format);
        info!("Swap chain width: {:#?}", swap_chain_desc.width);
        info!("Swap chain height: {:#?}", swap_chain_desc.height);
        info!(
            "Swap chain present mode: {:#?}",
            swap_chain_desc.present_mode
        );
        info!("MSAA samples: {:?}", config.graphics_config.msaa_mode);

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let frame_buffer = Self::create_multisampled_framebuffer(
            &device,
            &swap_chain_desc,
            config.graphics_config.msaa_mode as u32,
        );

        let depth_texture = device
            .create_texture(&TextureDescriptor {
                size: Extent3d {
                    width: swap_chain_desc.width,
                    height: swap_chain_desc.height,
                    depth: 1,
                },
                mip_level_count: 1,
                sample_count: config.graphics_config.msaa_mode as u32,
                dimension: TextureDimension::D2,
                format: Self::DEPTH_FORMAT,
                usage: TextureUsage::RENDER_ATTACHMENT,
                label: None,
            })
            .create_view(&TextureViewDescriptor::default());

        let vs_bytecode_path = "db/shaders/mipgen/final/blit.vert.spv";
        let fs_bytecode_path = "db/shaders/mipgen/final/blit.frag.spv";

        let vs_bytecode = std::fs::read(&vs_bytecode_path)
            .unwrap_or_else(|_| panic!("Failed to load vertex shader: {:?}", vs_bytecode_path));
        let fs_bytecode = std::fs::read(&fs_bytecode_path)
            .unwrap_or_else(|_| panic!("Failed to fragment shader: {:?}", fs_bytecode_path));

        let vs_module_desc = ShaderModuleDescriptor {
            label: None,
            source: util::make_spirv(&vs_bytecode[..]),
            flags: ShaderFlags::VALIDATION,
        };
        let fs_module_desc = ShaderModuleDescriptor {
            label: None,
            source: util::make_spirv(&fs_bytecode[..]),
            flags: ShaderFlags::VALIDATION,
        };

        let vs_module = device.create_shader_module(&vs_module_desc);
        let fs_module = device.create_shader_module(&fs_module_desc);
        let blit_shader = (vs_module, fs_module);

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
            depth_texture,
            msaa_samples: config.graphics_config.msaa_mode,
            blit_shader,
        }
    }

    pub fn generate_mipmaps(
        &self,
        encoder: &mut CommandEncoder,
        texture: &Texture,
        format: TextureFormat,
        mip_count: u32,
    ) {
        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("blit"),
                layout: None,
                vertex: VertexState {
                    module: &self.blit_shader.0,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(FragmentState {
                    module: &self.blit_shader.1,
                    entry_point: "main",
                    targets: &[format.into()],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
            });

        let bind_group_layout = pipeline.get_bind_group_layout(0);

        let sampler = self.device.create_sampler(&SamplerDescriptor {
            label: Some("mip"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let views = (0..mip_count)
            .map(|mip| {
                texture.create_view(&TextureViewDescriptor {
                    label: Some("mip"),
                    format: None,
                    dimension: None,
                    aspect: TextureAspect::All,
                    base_mip_level: mip,
                    level_count: std::num::NonZeroU32::new(1),
                    base_array_layer: 0,
                    array_layer_count: None,
                })
            })
            .collect::<Vec<_>>();

        for target_mip in 1..mip_count as usize {
            let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&views[target_mip - 1]),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&sampler),
                    },
                ],
                label: None,
            });

            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachmentDescriptor {
                    attachment: &views[target_mip],
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::WHITE),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.draw(0..4, 0..1);
        }
    }

    pub const REQUIRED_DEVICE_FEATURES: Features = Features::PUSH_CONSTANTS;
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    async fn create_async_resources(
        instance: &Instance,
        surface: &Surface,
        low_power_mode: bool,
        config: &GraphicsConfig,
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
                    features: Self::REQUIRED_DEVICE_FEATURES,
                    limits: Limits {
                        max_bind_groups: config.max_bind_groups,
                        max_dynamic_uniform_buffers_per_pipeline_layout: config
                            .max_dynamic_uniform_buffers_per_pipeline_layout,
                        max_dynamic_storage_buffers_per_pipeline_layout: config
                            .max_dynamic_storage_buffers_per_pipeline_layout,
                        max_sampled_textures_per_shader_stage: config
                            .max_sampled_textures_per_shader_stage,
                        max_samplers_per_shader_stage: config.max_samplers_per_shader_stage,
                        max_storage_buffers_per_shader_stage: config
                            .max_storage_buffers_per_shader_stage,
                        max_storage_textures_per_shader_stage: config
                            .max_storage_textures_per_shader_stage,
                        max_uniform_buffers_per_shader_stage: config
                            .max_uniform_buffers_per_shader_stage,
                        max_uniform_buffer_binding_size: config.max_uniform_buffer_binding_size,
                        max_push_constant_size: config.max_push_constant_pool_byte_size,
                    },
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
