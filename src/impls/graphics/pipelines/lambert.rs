use crate::impls::graphics::prelude::*;
use crate::resources::mesh::Vertex;
use wgpu::*;

pub fn create(drivers: &Drivers) -> ShaderPipeline {
    let bind_group_layout = [
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
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStage::FRAGMENT,
            ty: BindingType::Texture {
                multisampled: false,
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStage::FRAGMENT,
            ty: BindingType::Sampler {
                comparison: false,
                filtering: true,
            },
            count: None,
        },
    ];

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
        polygon_mode: PolygonMode::Fill
    };

    let multi_sample_state = MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false
    };

    drivers.create_shader_pipeline(ShaderPipelineDescriptor {
        modules: load_shader!("lambert"),
        bind_group_layouts: &bind_group_layout,
        push_constant_ranges: &[],
        primitive_state,
        depth_stencil: None,
        multi_sample_state,
        vertex_layouts: &[buffer_layout],
    })
}
