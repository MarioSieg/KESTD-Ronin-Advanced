use log::info;
use smallvec::{smallvec, SmallVec};
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
}

pub struct Frame<'a> {
    pub view: SwapChainTexture,
    pub encoder: CommandEncoder,
    pub queue: &'a Queue,
}

impl<'a> Frame<'a> {
    pub fn create_pass(&mut self) -> RenderPass {
        self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[RenderPassColorAttachmentDescriptor {
                attachment: &self.view.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLUE),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        })
    }

    pub fn end(self) {
        self.queue.submit(Some(self.encoder.finish()));
    }
}

pub struct ShaderBundle {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
    pub fs_targets: SmallVec<[ColorTargetState; 8]>,
}

impl ShaderBundle {
    pub fn vertex_state(&self) -> VertexState {
        VertexState {
            module: &self.vs_module,
            entry_point: SHADER_ENTRY,
            buffers: &[],
        }
    }

    pub fn fragment_state(&self) -> FragmentState {
        FragmentState {
            module: &self.fs_module,
            entry_point: SHADER_ENTRY,
            targets: &self.fs_targets[..],
        }
    }
}

pub const SHADER_ENTRY: &str = "main";

#[macro_export]
macro_rules! load_shader {
    ($name:literal) => {
        &(
            wgpu::include_spirv!(concat!(
                "../../db/shaders/fixed_pipelines/",
                $name,
                "/final/shader.vert.spv"
            )),
            wgpu::include_spirv!(concat!(
                "../../db/shaders/fixed_pipelines/",
                $name,
                "/final/shader.frag.spv"
            )),
        )
    };
}

impl Drivers {
    pub fn load_shader_bundle(
        &self,
        modules: &(ShaderModuleDescriptor, ShaderModuleDescriptor),
    ) -> ShaderBundle {
        let vs_module = self.device.create_shader_module(&modules.0);
        let fs_module = self.device.create_shader_module(&modules.1);
        let fs_targets = smallvec![self.swap_chain_format.into()];
        ShaderBundle {
            vs_module,
            fs_module,
            fs_targets,
        }
    }

    pub fn begin_frame(&self) -> Frame {
        let view = self
            .swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture!")
            .output;
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        Frame {
            view,
            encoder,
            queue: &self.queue,
        }
    }

    pub fn initialize(window: &glfw::Window, power_safe_mode: bool, vsync: bool) -> Self {
        let instance = Instance::new(BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let (adapter, device, queue) = futures::executor::block_on(Self::create_async_resources(
            &instance,
            &surface,
            power_safe_mode,
        ));

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

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

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
            swap_chain,
            swap_chain_desc,
            swap_chain_format,
        }
    }

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
