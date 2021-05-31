use crate::config::GraphicsConfig;
use wgpu::*;

pub const REQUIRED_DEVICE_FEATURES: Features = Features::PUSH_CONSTANTS;
pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

pub async fn create_async_resources(
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
                features: REQUIRED_DEVICE_FEATURES,
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

pub fn create_multi_sampled_framebuffer(
    device: &Device,
    sc_desc: &SwapChainDescriptor,
    sample_count: u32,
) -> TextureView {
    let multi_sampled_texture_extent = Extent3d {
        width: sc_desc.width,
        height: sc_desc.height,
        depth: 1,
    };

    let multi_sampled_frame_descriptor = &TextureDescriptor {
        size: multi_sampled_texture_extent,
        mip_level_count: 1,
        sample_count,
        dimension: TextureDimension::D2,
        format: sc_desc.format,
        usage: TextureUsage::RENDER_ATTACHMENT,
        label: None,
    };

    device
        .create_texture(multi_sampled_frame_descriptor)
        .create_view(&TextureViewDescriptor::default())
}
