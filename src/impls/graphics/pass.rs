use super::pipeline::ShaderPipeline;
use crate::resources::mesh::Mesh;
use std::sync::Arc;
use wgpu::*;

pub struct Pass<'a>(pub RenderPass<'a>);

impl<'a> Pass<'a> {
    #[inline]
    pub fn set_pipeline(&mut self, pipe: &'a ShaderPipeline) {
        self.0.set_pipeline(&pipe.render_pipeline);
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
