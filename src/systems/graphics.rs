use super::prelude::*;
use crate::ecs::components::{Camera, MeshRenderer, Transform};
use crate::ecs::resources::{CursorPos, Key, KeyInputQueue};
use crate::ecs::IntoQuery;
use crate::impls::graphics::matrix::CORRECTION_MATRIX;
use crate::impls::graphics::prelude::*;
use crate::impls::platform::prelude::WindowHandle;
use crate::math::{
    perspective, EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3,
};
use bytemuck::{Pod, Zeroable};
use cgmath::{Array, ElementWise, Vector2, VectorSpace};
use log::warn;
use wgpu::ShaderStage;

pub struct GraphicsSystem {
    pub drivers: Drivers,
    pub lambert_pipeline: LambertPipeline,
}

impl GraphicsSystem {
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        let width = self.drivers.swap_chain_desc.width as f32;
        let height = self.drivers.swap_chain_desc.height as f32;
        width / height
    }

    fn compute_camera(
        &self,
        camera_entity: (&mut Transform, &mut Camera),
        cursor_pos: CursorPos,
        key_queue: &KeyInputQueue,
    ) -> Matrix4<f32> {
        let trans: &mut Transform = camera_entity.0;
        let cam: &mut Camera = camera_entity.1;

        let dx = (cursor_pos.0 - cam.prev.x) / 300.0;
        let dy = (cursor_pos.1 - cam.prev.y) / 300.0;
        cam.prev = Vector2::new(cursor_pos.0, cursor_pos.1);

        cam.smooth_angles = cam
            .smooth_angles
            .lerp(Vector2::new(dx, dy), 1.0 / cam.smoothness);

        cam.angles.x -= dx + cam.smooth_angles.x;
        cam.angles.y += dy + cam.smooth_angles.y;
        cam.angles.y = cam.angles.y.clamp(-cam.clamp_y, cam.clamp_y);

        let x = Rad(cam.angles.y).0.cos() * Rad(cam.angles.x).0.sin();
        let y = Rad(cam.angles.y).0.sin();
        let z = Rad(cam.angles.y).0.cos() * Rad(cam.angles.x).0.cos();

        let forward = Vector3::new(x, y, z).normalize();
        let left = forward.cross(Vector3::unit_y()).normalize();

        let mut eye = trans.position;

        if key_queue.is_key_pressed(Key::W) {
            eye += Vector3::from_value(cam.speed).mul_element_wise(forward);
        }

        if key_queue.is_key_pressed(Key::A) {
            eye -= Vector3::from_value(cam.speed).mul_element_wise(left);
        }

        if key_queue.is_key_pressed(Key::S) {
            eye -= Vector3::from_value(cam.speed).mul_element_wise(forward);
        }

        if key_queue.is_key_pressed(Key::D) {
            eye += Vector3::from_value(cam.speed).mul_element_wise(left);
        }

        let at = eye + forward;

        trans.position = eye;

        let projection_matrix =
            perspective(cam.fov, self.aspect_ratio(), cam.near_clip, cam.far_clip);

        let view_matrix = Matrix4::look_at_rh(
            Point3::from_vec(eye),
            Point3::from_vec(at),
            Vector3::unit_y(),
        );

        CORRECTION_MATRIX * projection_matrix * view_matrix
    }
}

impl SubSystem for GraphicsSystem {
    type Args = WindowHandle;

    fn initialize(cfg: &mut CoreConfig, window: &Self::Args) -> Self {
        let drivers = Drivers::initialize(window, cfg);
        let lambert_pipeline = LambertPipeline::create(&drivers, cfg);

        Self {
            drivers,
            lambert_pipeline,
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
                let key_queue = scenery.resources.get::<KeyInputQueue>().unwrap();
                self.compute_camera(camera, cursor_pos, &*key_queue)
            } else {
                warn!("No camera found!");
                flag = false;
                Matrix4::identity()
            };

            let mut pass = frame.create_pass(true);
            pass.set_pipeline(&self.lambert_pipeline);

            let mut query = <(&Transform, &MeshRenderer)>::query();
            query.for_each_mut(&mut scenery.world, |(transform, renderer)| {
                let world_matrix = transform.calculate_matrix();
                let push_constant_data = PushConstantData {
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

        frame.end();
        flag
    }
}

#[derive(Copy, Clone)]
struct PushConstantData {
    pub world_matrix: Matrix4<f32>,
    pub view_proj_matrix: Matrix4<f32>,
}

unsafe impl Pod for PushConstantData {}
unsafe impl Zeroable for PushConstantData {}
