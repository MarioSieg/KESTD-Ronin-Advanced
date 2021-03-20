use super::Drivers;
use smallvec::{smallvec, SmallVec};
use wgpu::*;

pub struct ShaderPipeline {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
    pub fs_targets: SmallVec<[ColorTargetState; 8]>,
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,
    pub bind_group_layout: BindGroupLayout
}

impl ShaderPipeline {
    pub fn create_shader_bundle(drivers: &Drivers, desc: ShaderPipelineDescriptor) -> Self {
        let vs_module = drivers.device.create_shader_module(&desc.modules.0);
        let fs_module = drivers.device.create_shader_module(&desc.modules.1);
        let fs_targets = smallvec![drivers.swap_chain_format.into()];
        let bind_group_layout =
            drivers
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: desc.bind_group_layouts,
                });
        let pipeline_layout = drivers
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout][..],
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
                    buffers: desc.vertex_layouts,
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: SHADER_ENTRY,
                    targets: &fs_targets[..],
                }),
                primitive: desc.primitive_state,
                depth_stencil: desc.depth_stencil,
                multisample: desc.multi_sample_state,
            });
        Self {
            vs_module,
            fs_module,
            fs_targets,
            pipeline_layout,
            render_pipeline,
            bind_group_layout
        }
    }
}

pub struct ShaderPipelineDescriptor<'a> {
    pub modules: &'a (ShaderModuleDescriptor<'a>, ShaderModuleDescriptor<'a>),
    pub bind_group_layouts: &'a [BindGroupLayoutEntry],
    pub push_constant_ranges: &'a [PushConstantRange],
    pub primitive_state: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multi_sample_state: MultisampleState,
    pub vertex_layouts: &'a [VertexBufferLayout<'a>],
}

pub const SHADER_ENTRY: &str = "main";

#[macro_export]
macro_rules! load_shader {
    ($name:literal) => {
        &(
            wgpu::include_spirv!(concat!(
                "../../../../db/shaders/fixed_pipelines/",
                $name,
                "/final/shader.vert.spv"
            )),
            wgpu::include_spirv!(concat!(
                "../../../../db/shaders/fixed_pipelines/",
                $name,
                "/final/shader.frag.spv"
            )),
        )
    };
}
