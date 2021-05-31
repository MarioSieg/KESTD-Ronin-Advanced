use super::prelude::*;

pub struct LambertPipeline {
    pub shader_pipeline: ShaderPipeline,
}

#[derive(Copy, Clone)]
pub struct PushConstantData {
    pub world_matrix: Matrix4<f32>,
    pub view_proj_matrix: Matrix4<f32>,
}

unsafe impl Pod for PushConstantData {}
unsafe impl Zeroable for PushConstantData {}

impl Pipeline for LambertPipeline {
    type PushConstantData = PushConstantData;

    const NAME: &'static str = "Lambert";

    const IS_SURFACE_PIPELINE: bool = true;

    const PER_MATERIAL_BIND_GROUP_LAYOUT_ENTRIES: &'static [BindGroupLayoutEntry] = &[
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
        // 2 * mat4x4 - word matrix, view projection matrix
        PushConstantRange {
            stages: ShaderStage::VERTEX,
            range: (0..128),
        },
    ];

    const DEPTH_STENCIL_STATE: Option<DepthStencilState> = Some(DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: CompareFunction::Less,
        stencil: StencilState {
            front: StencilFaceState::IGNORE,
            back: StencilFaceState::IGNORE,
            write_mask: 0,
            read_mask: 0,
        },
        bias: DepthBiasState {
            constant: 0,
            slope_scale: 0.0,
            clamp: 0.0,
        },
        clamp_depth: false,
    });

    #[inline]
    fn shader_pipeline(&self) -> &ShaderPipeline {
        &self.shader_pipeline
    }

    fn create(drivers: &mut Drivers, config: &CoreConfig) -> Self {
        let multi_sample_state = MultisampleState {
            count: config.graphics_config.msaa_mode as u32,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let shader_pipeline =
            drivers.create_shader_pipeline::<Self>(ShaderPipelineDescriptor { multi_sample_state });

        Self { shader_pipeline }
    }
}
