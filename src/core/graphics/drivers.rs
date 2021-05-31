use super::boot;
use super::frame::Frame;
use super::mipgen;
use super::pipeline::{Pipeline, ShaderPipeline, ShaderPipelineDescriptor};
use super::shader_compiler;
use crate::config::{CoreConfig, GraphicsApi, MsaaMode};
use log::info;
use shaderc::{CompilationArtifact, Compiler as ShaderCompiler, ShaderKind};
use std::path::PathBuf;
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
    pub shader_compiler: ShaderCompiler,
    pub blit_shader: (ShaderModule, ShaderModule),
}

impl Drivers {
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        let width = self.swap_chain_desc.width as f32;
        let height = self.swap_chain_desc.height as f32;
        width / height
    }

    pub fn create_shader_pipeline<T: Pipeline>(
        &mut self,
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

    pub fn compile_shader_raw(&mut self, path: PathBuf, kind: ShaderKind) -> CompilationArtifact {
        shader_compiler::compile_to_bytecode(self, path, kind)
    }

    pub fn compile_and_create_shader(&mut self, path: PathBuf, kind: ShaderKind) -> ShaderModule {
        let code = self.compile_shader_raw(path, kind);
        // bug in wgpu - shader validation fails on vertex shaders with push constants
        let flags = if kind == ShaderKind::Vertex {
            ShaderFlags::default()
        } else {
            ShaderFlags::VALIDATION
        };
        let desc = ShaderModuleDescriptor {
            label: None,
            source: util::make_spirv(code.as_binary_u8()),
            flags,
        };
        self.device.create_shader_module(&desc)
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
        let (adapter, device, queue) = futures::executor::block_on(boot::create_async_resources(
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

        let frame_buffer = boot::create_multi_sampled_framebuffer(
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
                format: boot::DEPTH_FORMAT,
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

        let shader_compiler = shaderc::Compiler::new().expect("Failed to create shader compiler!");

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
            shader_compiler,
            blit_shader,
        }
    }

    #[inline]
    pub fn generate_mipmaps(
        &self,
        encoder: &mut CommandEncoder,
        texture: &Texture,
        format: TextureFormat,
        mip_count: u32,
    ) {
        mipgen::generate_mipmaps(self, encoder, texture, format, mip_count);
    }
}
