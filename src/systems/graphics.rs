use super::prelude::*;
use crate::ecs::components::MeshRenderer;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use crate::resources::mesh::Mesh;
use crate::resources::texture::Texture;
use crate::resources::ResourceImporteur;
use std::path::PathBuf;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: LambertPipeline,
    pub bind_group: Option<wgpu::BindGroup>,

    renderer: Option<MeshRenderer>,
}

impl SubSystem for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let is_power_safe_mode = cfg.application_config.power_safe_mode;
        let use_vsync = cfg.display_config.vsync;

        let drivers = Drivers::initialize(
            window,
            is_power_safe_mode,
            use_vsync,
            cfg.graphics_config.msaa_mode,
        );

        let lambert_pipeline = LambertPipeline::create(&drivers, cfg);

        let mut this = Self {
            drivers,
            lambert_pipeline,
            bind_group: None,
            renderer: None,
        };

        this.renderer = Some(MeshRenderer {
            mesh: Mesh::load(&this, PathBuf::from("db/meshes/cube.obj")),
            texture: Texture::load(&this, PathBuf::from("db/textures/grid.png")),
        });

        let bind_group = this
            .drivers
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &this
                    .lambert_pipeline
                    .shader_pipeline
                    .material_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            this.renderer.as_ref().unwrap().texture.view(),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            this.renderer.as_ref().unwrap().texture.sampler(),
                        ),
                    },
                ],
                label: None,
            });

        this.bind_group = Some(bind_group);

        this
    }

    fn tick(&mut self) -> bool {
        let mut frame = self.drivers.begin_frame();
        {
            let mut pass = frame.create_pass();
            pass.set_pipeline(&self.lambert_pipeline);
            pass.set_bind_group(
                0,
                &self.lambert_pipeline.shader_pipeline.internal_bind_group,
            );
            pass.set_bind_group(1, self.bind_group.as_ref().unwrap());
            pass.draw_indexed(&self.renderer.as_ref().unwrap().mesh);
        }

        frame.end();
        true
    }
}
