use super::Drivers;
use crate::config::CoreConfig;
use bytemuck::{Pod, Zeroable};
use log::info;
use shaderc::{CompileOptions, ShaderKind};
use smallvec::{smallvec, SmallVec};
use std::fs;
use wgpu::*;

pub trait Pipeline {
    type PushConstantData: Copy + Clone + Pod + Zeroable;
    const NAME: &'static str;
    const IS_SURFACE_PIPELINE: bool;
    const PER_MATERIAL_BIND_GROUP_LAYOUT_ENTRIES: &'static [BindGroupLayoutEntry];
    const PRIMITIVE_STATE: PrimitiveState;
    const VERTEX_BUFFER_LAYOUTS: &'static [VertexBufferLayout<'static>];
    const PUSH_CONSTANT_RANGES: &'static [PushConstantRange];
    const DEPTH_STENCIL_STATE: Option<DepthStencilState>;

    fn shader_pipeline(&self) -> &ShaderPipeline;
    fn create(_drivers: &mut Drivers, _config: &CoreConfig) -> Self;
}

pub struct ShaderPipeline {
    pub vs_module: ShaderModule,
    pub fs_module: ShaderModule,
    pub fs_targets: SmallVec<[ColorTargetState; 8]>,
    pub pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,
    pub per_material_bind_group_layout: BindGroupLayout,
}

pub struct ShaderPipelineDescriptor {
    pub multi_sample_state: MultisampleState,
}

impl ShaderPipeline {
    pub fn create_shader_bundle<T: Pipeline>(
        drivers: &mut Drivers,
        desc: ShaderPipelineDescriptor,
    ) -> Self {
        let name = String::from(T::NAME).to_lowercase();

        let vs_bytecode_path = format!("db/shaders/fixed_pipelines/{}/shader.vert.glsl", name);
        let fs_bytecode_path = format!("db/shaders/fixed_pipelines/{}/shader.frag.glsl", name);

        let mut compile_bytecode = |path: String, shader_type: ShaderKind| {
            let source = fs::read_to_string(&path).unwrap_or_else(|_| {
                panic!("Failed to load file: {}", path);
            });
            let sh_opt = &CompileOptions::new().expect("Failed to create compile options!");
            info!("Compiling shader: {}", path);
            drivers
                .shader_compiler
                .compile_into_spirv(
                    source.as_str(),
                    shader_type,
                    path.as_str(),
                    "main",
                    Some(sh_opt),
                )
                .unwrap_or_else(|_| {
                    panic!("Failed to compile shader: {}", path);
                })
        };

        let vs_bytecode = compile_bytecode(vs_bytecode_path, shaderc::ShaderKind::Vertex);
        let fs_bytecode = compile_bytecode(fs_bytecode_path, shaderc::ShaderKind::Fragment);

        let vs_module_desc = ShaderModuleDescriptor {
            label: None,
            source: util::make_spirv(vs_bytecode.as_binary_u8()),
            flags: ShaderFlags::default(),
        };
        let fs_module_desc = ShaderModuleDescriptor {
            label: None,
            source: util::make_spirv(fs_bytecode.as_binary_u8()),
            flags: ShaderFlags::VALIDATION,
        };

        let vs_module = drivers.device.create_shader_module(&vs_module_desc);
        let fs_module = drivers.device.create_shader_module(&fs_module_desc);
        let fs_targets = smallvec![drivers.swap_chain_format.into()];

        let material_bind_group_layout =
            drivers
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: T::PER_MATERIAL_BIND_GROUP_LAYOUT_ENTRIES,
                });

        let pipeline_layout = drivers
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&material_bind_group_layout][..],
                push_constant_ranges: T::PUSH_CONSTANT_RANGES,
            });

        let render_pipeline = drivers
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &vs_module,
                    entry_point: Self::SHADER_ENTRY,
                    buffers: T::VERTEX_BUFFER_LAYOUTS,
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: Self::SHADER_ENTRY,
                    targets: &fs_targets[..],
                }),
                primitive: T::PRIMITIVE_STATE,
                depth_stencil: T::DEPTH_STENCIL_STATE,
                multisample: desc.multi_sample_state,
            });

        Self {
            vs_module,
            fs_module,
            fs_targets,
            pipeline_layout,
            render_pipeline,
            per_material_bind_group_layout: material_bind_group_layout,
        }
    }

    pub const SHADER_ENTRY: &'static str = "main";
}
