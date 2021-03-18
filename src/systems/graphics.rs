use super::System;
use crate::config::CoreConfig;
use log::info;

pub struct GraphicsSystem {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
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
        }
    }

    fn tick(&mut self) -> bool {
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
