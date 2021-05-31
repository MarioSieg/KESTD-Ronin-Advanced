use super::pass::Pass;
use crate::config::MsaaMode;
use wgpu::*;

pub struct Frame<'a> {
    pub view: SwapChainTexture,
    pub encoder: CommandEncoder,
    pub queue: &'a Queue,
    pub frame_buf: &'a TextureView,
    pub depth_stencil: &'a TextureView,
    pub(super) samples: MsaaMode,
}

impl<'a> Frame<'a> {
    pub fn create_pass(&mut self, use_depth_stencil: bool) -> Pass {
        let ops = Operations {
            load: LoadOp::Clear(Color::WHITE),
            store: true,
        };
        let color_attachment = if self.samples == MsaaMode::Off {
            RenderPassColorAttachmentDescriptor {
                attachment: &self.view.view,
                resolve_target: None,
                ops,
            }
        } else {
            RenderPassColorAttachmentDescriptor {
                attachment: self.frame_buf,
                resolve_target: Some(&self.view.view),
                ops,
            }
        };
        let render_pass = self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[color_attachment],
            depth_stencil_attachment: if use_depth_stencil {
                Some(RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_stencil,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                })
            } else {
                None
            },
        });
        Pass(render_pass)
    }

    pub fn end(self) {
        self.queue.submit(Some(self.encoder.finish()));
    }
}
