use super::pipeline::Pipeline;
use crate::resources::mesh::Mesh;
use std::sync::Arc;
use wgpu::*;

pub struct Pass<'a>(pub RenderPass<'a>);

impl<'a> Pass<'a> {
    #[inline]
    pub fn set_push_constans(&mut self, stage: ShaderStage, offset: u32, data: &[u8]) {
        self.0.set_push_constants(stage, offset, data)
    }

    #[inline]
    pub fn set_pipeline<T: Pipeline>(&mut self, pipe: &'a T) {
        self.0.set_pipeline(&pipe.shader_pipeline().render_pipeline);
    }

    #[inline]
    pub fn set_bind_group(&mut self, index: u32, group: &'a BindGroup) {
        self.0.set_bind_group(index, group, &[]);
    }

    pub fn draw_indexed(&mut self, mesh: &'a Arc<Mesh>) {
        self.0
            .set_index_buffer(mesh.index_buffer().slice(..), IndexFormat::Uint16);
        self.0.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
        self.0.draw_indexed(0..mesh.indices().len() as u32, 0, 0..1)
    }
}
