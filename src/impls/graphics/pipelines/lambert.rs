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

    const INTERNAL_BIND_GROUP_LAYOUT_ENTRIES: &'static [BindGroupLayoutEntry] =
        &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStage::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(64),
            },
            count: None,
        }];

    const MATERIAL_BIND_GROUP_LAYOUT_ENTRIES: &'static [BindGroupLayoutEntry] = &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStage::FRAGMENT,
            ty: BindingType::Texture {
                multisampled: false,
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStage::FRAGMENT,
            ty: BindingType::Sampler {
                comparison: false,
                filtering: true,
            },
            count: None,
        },
    ];

    const PRIMITIVE_STATE: PrimitiveState = PrimitiveState {
        topology: PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: FrontFace::Ccw,
        cull_mode: CullMode::Back,
        polygon_mode: PolygonMode::Fill,
    };

    const VERTEX_BUFFER_LAYOUTS: &'static [VertexBufferLayout<'static>] = &[VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::InputStepMode::Vertex,
        attributes: &vertex_attr_array![
            0 => Float4,
            1 => Float2
        ],
    }];

    #[inline]
    fn shader_pipeline(&self) -> &ShaderPipeline {
        &self.shader_pipeline
    }

    fn create(drivers: &Drivers) -> Self {
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

        let internal_bind_group_entries = &[BindGroupEntry {
            binding: 0,
            resource: view_projection_buffer.as_entire_binding(),
        }][..];

        let shader_pipeline = drivers.create_shader_pipeline::<Self>(ShaderPipelineDescriptor {
            depth_stencil: None,
            multi_sample_state,
            internal_bind_group_entries,
        });

        Self { shader_pipeline }
    }
}
