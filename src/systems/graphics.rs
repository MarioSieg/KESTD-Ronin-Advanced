use super::prelude::*;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use crate::resources::mesh::Mesh;
use crate::resources::texture::Texture;
use crate::resources::ResourceImporteur;
use std::path::PathBuf;
use std::sync::Arc;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: LambertPipeline,

    pub bind_group: Option<wgpu::BindGroup>,
    mesh: Option<Arc<Mesh>>,
    texture: Option<Arc<Texture>>,
}

impl System for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(window, is_power_safe_mode, use_vsync);

        let lambert_pipeline = LambertPipeline::create(&drivers);

        let mut this = Self {
            drivers,
            lambert_pipeline,
            bind_group: None,
            mesh: None,
            texture: None,
        };

        let mesh = Mesh::load(&this, PathBuf::from("db/meshes/cube.obj"));
        let texture = Texture::load(&this, PathBuf::from("db/textures/grid.png"));

        let bind_group = this
            .drivers
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &this.lambert_pipeline.shader_pipeline.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: this
                            .lambert_pipeline
                            .view_projection_buffer
                            .as_entire_binding(),
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
            pass.set_pipeline(&self.lambert_pipeline);
            pass.set_bind_group(0, self.bind_group.as_ref().unwrap());
            pass.draw_indexed(self.mesh.as_ref().unwrap());
        }

        frame.end();
        true
    }
}
