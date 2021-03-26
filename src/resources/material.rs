use super::prelude::*;
use super::texture::Texture;
use crate::systems::graphics::GraphicsSystem;
use wgpu::BindGroup;

pub enum MaterialProperties {
    Lambert { albedo: Arc<Texture> },
}

pub struct Material {
    properties: MaterialProperties,
    bind_group: wgpu::BindGroup,
}

impl Material {
    #[inline]
    pub fn properties(&self) -> &MaterialProperties {
        &self.properties
    }

    #[inline]
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn load(system: &GraphicsSystem, properties: MaterialProperties) -> Arc<Self> {
        use wgpu::*;

        let bind_group = match &properties {
            MaterialProperties::Lambert { albedo } => {
                system
                    .drivers
                    .device
                    .create_bind_group(&BindGroupDescriptor {
                        layout: &system
                            .lambert_pipeline
                            .shader_pipeline
                            .per_material_bind_group_layout,
                        entries: &[
                            BindGroupEntry {
                                binding: 0,
                                resource: BindingResource::TextureView(albedo.view()),
                            },
                            BindGroupEntry {
                                binding: 1,
                                resource: BindingResource::Sampler(albedo.sampler()),
                            },
                        ],
                        label: None,
                    })
            }
        };

        Arc::new(Self {
            properties,
            bind_group,
        })
    }
}
