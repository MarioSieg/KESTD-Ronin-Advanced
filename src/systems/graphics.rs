use super::prelude::*;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use crate::resources::mesh::Mesh;
use crate::resources::ResourceImporteur;
use std::path::PathBuf;
use wgpu::IndexFormat;
use crate::impls::graphics::matrix::generate_matrix;
use crate::resources::texture::Texture;
use std::sync::Arc;

use wgpu::util::DeviceExt;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: ShaderPipeline,
    pub matrix_buffer: wgpu::Buffer,
    pub bind_group: Option<wgpu::BindGroup>,
    mesh: Option<Arc<Mesh>>,
    texture: Option<Arc<Texture>>
}

impl System for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(window, is_power_safe_mode, use_vsync);

        let lambert_pipeline = pipelines::lambert::create(&drivers);

        let mx_total = generate_matrix(drivers.swap_chain_desc.width as f32 / drivers.swap_chain_desc.height as f32);
        let mx_ref: &[f32; 16] = mx_total.as_ref();
        let matrix_buffer = drivers.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let mut this = Self {
            drivers,
            lambert_pipeline,
            matrix_buffer,
            bind_group: None,
            mesh: None,
            texture: None
        };

        let mesh = Mesh::load(&this, PathBuf::from(""));
        let texture = Texture::load(&this, PathBuf::from(""));

        let bind_group = this.drivers.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &this.lambert_pipeline.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: this.matrix_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(texture.sampler()),
                },
            ],
            label: None,
        });

        this.mesh = Some(mesh);
        this.texture = Some(texture);
        this.bind_group = Some(bind_group);

        this
    }

    fn tick(&mut self) -> bool {

        let mut frame = self.drivers.begin_frame();
        {
            let mut pass = frame.create_pass();
            pass.set_pipeline(&self.lambert_pipeline.render_pipeline);
            pass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);
            pass.set_index_buffer(self.mesh.as_ref().unwrap().index_buffer().slice(..), IndexFormat::Uint16);
            pass.set_vertex_buffer(0, self.mesh.as_ref().unwrap().vertex_buffer().slice(..));
            pass.draw_indexed(0..self.mesh.as_ref().unwrap().indices().len() as u32, 0, 0..1)
        }

        frame.end();
        true
    }
}
