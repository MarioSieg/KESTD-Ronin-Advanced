use super::prelude::*;
use crate::components::{Camera, MeshRenderer, Transform};
use crate::core::graphics::{
    camera, drivers::Drivers, gui::Renderer as ImGuiRenderer,
    gui::RendererConfig as ImGuiRendererConfig, pipeline::Pipeline, pipelines::lambert,
};
use crate::core::platform::prelude::WindowHandle;
use crate::scenery_resources::{KeyInputStateCollection, MouseInputStateCollection};
use cgmath::{Matrix4, SquareMatrix};
use legion::IntoQuery;
use log::warn;
use wgpu::ShaderStage;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: lambert::LambertPipeline,
    pub imgui: imgui::Context,
    pub imgui_renderer: ImGuiRenderer,
}

impl SubSystem for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let mut drivers = Drivers::initialize(window, cfg);
        let lambert_pipeline = lambert::LambertPipeline::create(&mut drivers, cfg);

        let mut imgui = imgui::Context::create();

        imgui.set_ini_filename(None);

        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: 16.0,
                    ..Default::default()
                }),
            }]);

        let io = imgui.io_mut();
        io.display_size[0] = cfg.display_config.resolution.0 as f32;
        io.display_size[1] = cfg.display_config.resolution.1 as f32;

        let imgui_renderer_config = ImGuiRendererConfig {
            texture_format: drivers.swap_chain_format,
            sample_count: drivers.msaa_samples as u32,
            ..Default::default()
        };
        let imgui_renderer = ImGuiRenderer::new(
            &mut imgui,
            &drivers.device,
            &drivers.queue,
            imgui_renderer_config,
        );

        Self {
            drivers,
            lambert_pipeline,
            imgui,
            imgui_renderer,
        }
    }

    fn tick(&mut self, scenery: &mut Scenery) -> bool {
        let mut flag = true;
        let mut frame = self.drivers.begin_frame();
        {
            let camera = <(&mut Transform, &mut Camera)>::query()
                .iter_mut(&mut scenery.world)
                .next();
            let view_proj_matrix = if let Some(camera) = camera {
                let cursor_pos = *scenery.resources.get_mut_or_default();
                let key_queue = scenery.resources.get::<KeyInputStateCollection>().unwrap();
                let mouse_queue = scenery
                    .resources
                    .get::<MouseInputStateCollection>()
                    .unwrap();
                camera::compute_camera(
                    self.drivers.aspect_ratio(),
                    camera,
                    cursor_pos,
                    &*key_queue,
                    &*mouse_queue,
                )
            } else {
                warn!("No camera found!");
                flag = false;
                Matrix4::identity()
            };

            // draw 3d scene:
            {
                let mut pass = frame.create_pass(true);
                pass.set_pipeline(&self.lambert_pipeline);

                let mut render_query = <(&Transform, &MeshRenderer)>::query();
                render_query.for_each(&scenery.world, |(transform, renderer)| {
                    let world_matrix = transform.calculate_matrix();
                    let push_constant_data = lambert::PushConstantData {
                        world_matrix,
                        view_proj_matrix,
                    };
                    pass.set_push_constans(
                        ShaderStage::VERTEX,
                        0,
                        bytemuck::bytes_of(&push_constant_data),
                    );
                    pass.set_bind_group(0, renderer.material.bind_group());
                    pass.draw_indexed(&renderer.mesh);
                });
            }

            // draw gui:
            {
                let mut pass = frame
                    .encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                let ui = self.imgui.frame();
                {
                    let mut opened = true;
                    ui.show_demo_window(&mut opened);
                }
                self.imgui_renderer
                    .render(
                        ui.render(),
                        &self.drivers.queue,
                        &self.drivers.device,
                        &mut pass,
                    )
                    .expect("GUI rendering failed");
            }
        }

        frame.end();
        flag
    }
}
