use super::drivers::Drivers;
use wgpu::*;

pub fn generate_mipmaps(
    drivers: &Drivers,
    encoder: &mut CommandEncoder,
    texture: &Texture,
    format: TextureFormat,
    mip_count: u32,
) {
    let pipeline = drivers
        .device
        .create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("blit"),
            layout: None,
            vertex: VertexState {
                module: &drivers.blit_shader.0,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &drivers.blit_shader.1,
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

    let sampler = drivers.device.create_sampler(&SamplerDescriptor {
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
        let bind_group = drivers.device.create_bind_group(&BindGroupDescriptor {
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
