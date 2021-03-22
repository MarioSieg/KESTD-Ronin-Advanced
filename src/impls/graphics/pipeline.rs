use super::Drivers;
use smallvec::{smallvec, SmallVec};
use wgpu::*;

pub struct ShaderPipeline {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
    pub fs_targets: SmallVec<[ColorTargetState; 8]>,
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,
    pub internal_bind_group_layout: BindGroupLayout,
    pub public_bind_group_layout: BindGroupLayout,
    pub internal_bind_group: BindGroup,
}

pub struct ShaderPipelineDescriptor<'a> {
    pub modules: &'a (ShaderModuleDescriptor<'a>, ShaderModuleDescriptor<'a>),
    pub push_constant_ranges: &'a [PushConstantRange],
    pub primitive_state: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multi_sample_state: MultisampleState,
    pub vertex_layouts: &'a [VertexBufferLayout<'a>],
    pub internal_bind_group_layout_entries: &'a [BindGroupLayoutEntry],
    pub public_bind_group_layout_entries: &'a [BindGroupLayoutEntry],
    pub internal_bind_group_entries: &'a [BindGroupEntry<'a>],
}

impl ShaderPipeline {
    pub fn create_shader_bundle(drivers: &Drivers, desc: ShaderPipelineDescriptor) -> Self {
        let vs_module = drivers.device.create_shader_module(&desc.modules.0);
        let fs_module = drivers.device.create_shader_module(&desc.modules.1);
        let fs_targets = smallvec![drivers.swap_chain_format.into()];

        let internal_bind_group_layout =
            drivers
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: desc.internal_bind_group_layout_entries,
                });

        let internal_bind_group = drivers.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &internal_bind_group_layout,
            entries: desc.internal_bind_group_entries
        });

        let public_bind_group_layout = drivers.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: desc.public_bind_group_layout_entries
        });

        let pipeline_layout = drivers
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&internal_bind_group_layout, &public_bind_group_layout][..],
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
            internal_bind_group_layout,
            internal_bind_group,
            public_bind_group_layout
        }
    }
}

pub const SHADER_ENTRY: &str = "main";

pub trait Pipeline {
    const NAME: &'static str;
    const IS_SURFACE_PIPELINE: bool;

    fn shader_pipeline(&self) -> &ShaderPipeline;
    fn create(drivers: &Drivers) -> Self;
}

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
