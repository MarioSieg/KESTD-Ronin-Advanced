use super::Drivers;
use smallvec::{smallvec, SmallVec};
use wgpu::*;

pub struct ShaderPipeline {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
    pub fs_targets: SmallVec<[ColorTargetState; 8]>,
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,
}

impl ShaderPipeline {
    pub fn create_shader_bundle(drivers: &Drivers, desc: ShaderPipelineDescriptor) -> Self {
        let vs_module = drivers.device.create_shader_module(&desc.modules.0);
        let fs_module = drivers.device.create_shader_module(&desc.modules.1);
        let fs_targets = smallvec![drivers.swap_chain_format.into()];
        let pipeline_layout = drivers
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: desc.bind_group_layouts,
                push_constant_ranges: desc.push_constant_ranges,
            });
        let render_pipeline = drivers
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &vs_module,
                    entry_point: SHADER_ENTRY,
                    buffers: &[],
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: SHADER_ENTRY,
                    targets: &fs_targets[..],
                }),
                primitive: desc.primitive_state,
                depth_stencil: desc.depth_stencil,
                multisample: desc.multisample,
            });
        Self {
            vs_module,
            fs_module,
            fs_targets,
            pipeline_layout,
            render_pipeline,
        }
    }
}

pub struct ShaderPipelineDescriptor<'a> {
    pub modules: &'a (ShaderModuleDescriptor<'a>, ShaderModuleDescriptor<'a>),
    pub bind_group_layouts: &'a [&'a BindGroupLayout],
    pub push_constant_ranges: &'a [PushConstantRange],
    pub primitive_state: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
}

impl<'a> ShaderPipelineDescriptor<'a> {
    pub fn new_simple(
        modules: &'a (ShaderModuleDescriptor<'a>, ShaderModuleDescriptor<'a>),
    ) -> Self {
        Self {
            modules,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
            primitive_state: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
        }
    }
}

pub const SHADER_ENTRY: &str = "main";

#[macro_export]
macro_rules! load_shader {
    ($name:literal) => {
        &(
            wgpu::include_spirv!(concat!(
                "../../db/shaders/fixed_pipelines/",
                $name,
                "/final/shader.vert.spv"
            )),
            wgpu::include_spirv!(concat!(
                "../../db/shaders/fixed_pipelines/",
                $name,
                "/final/shader.frag.spv"
            )),
        )
    };
}
