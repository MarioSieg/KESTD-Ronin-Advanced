use crate::impls::graphics::prelude::*;
use crate::resources::mesh::Vertex;
use wgpu::*;

pub struct LambertPipeline {
    pub shader_pipeline: ShaderPipeline,
}

impl Pipeline for LambertPipeline {
    const NAME: &'static str = "Lambert";

    const IS_SURFACE_PIPELINE: bool = true;

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
        array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
        step_mode: InputStepMode::Vertex,
        attributes: &vertex_attr_array![
            0 => Float4,
            1 => Float2
        ],
    }];

    const PUSH_CONSTANT_RANGES: &'static [PushConstantRange] = &[
        // mat4x4 - transform
        PushConstantRange {
            stages: ShaderStage::VERTEX,
            range: (0..64),
        },
    ];

    #[inline]
    fn shader_pipeline(&self) -> &ShaderPipeline {
        &self.shader_pipeline
    }

    fn create(drivers: &Drivers, config: &CoreConfig) -> Self {
        let multi_sample_state = MultisampleState {
            count: config.graphics_config.msaa_mode as u32,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let shader_pipeline = drivers.create_shader_pipeline::<Self>(ShaderPipelineDescriptor {
            depth_stencil: None,
            multi_sample_state,
        });

        Self { shader_pipeline }
    }
}
