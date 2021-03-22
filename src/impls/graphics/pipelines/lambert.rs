use crate::impls::graphics::matrix::generate_matrix;
use crate::impls::graphics::prelude::*;
use crate::resources::mesh::Vertex;
use util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

pub struct LambertPipeline {
    pub shader_pipeline: ShaderPipeline,
}

impl Pipeline for LambertPipeline {
    const NAME: &'static str = "Lambert";
    const IS_SURFACE_PIPELINE: bool = true;

    #[inline]
    fn shader_pipeline(&self) -> &ShaderPipeline {
        &self.shader_pipeline
    }

    fn create(drivers: &Drivers) -> Self {
        let buffer_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &vertex_attr_array![
                0 => Float4,
                1 => Float2
            ],
        };

        let primitive_state = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            polygon_mode: PolygonMode::Fill,
        };

        let multi_sample_state = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let mx_total = generate_matrix(
            drivers.swap_chain_desc.width as f32 / drivers.swap_chain_desc.height as f32,
        );
        let mx_ref: &[f32; 16] = mx_total.as_ref();
        let view_projection_buffer = drivers.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let internal_bind_group_layout_entries = &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(64),
                },
                count: None,
            },
        ];

        let internal_bind_group_entries = &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource:
                    view_projection_buffer
                    .as_entire_binding(),
            },
        ][..];

        let public_bind_group_layout_entries = &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                    filtering: true,
                },
                count: None,
            },
        ];


        let shader_pipeline = drivers.create_shader_pipeline(ShaderPipelineDescriptor {
            modules: load_shader!("lambert"),
            push_constant_ranges: &[],
            primitive_state,
            depth_stencil: None,
            multi_sample_state,
            vertex_layouts: &[buffer_layout],
            internal_bind_group_layout_entries,
            internal_bind_group_entries,
            public_bind_group_layout_entries
        });

        Self {
            shader_pipeline,
        }
    }
}
