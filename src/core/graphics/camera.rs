use crate::components::{Camera, Transform};
use crate::scenery_resources::{
    CursorPos, Key, KeyInputStateCollection, MouseButton, MouseInputStateCollection,
};
use cgmath::*;

pub fn compute_camera(
    aspect_ratio: f32,
    camera_entity: (&mut Transform, &mut Camera),
    cursor_pos: CursorPos,
    key_state: &KeyInputStateCollection,
    mouse_state: &MouseInputStateCollection,
) -> Matrix4<f32> {
    let trans: &mut Transform = camera_entity.0;
    let cam: &mut Camera = camera_entity.1;

    if mouse_state.is_key_pressed(MouseButton::Button2) || cam.prev.is_zero() {
        let dx = (cursor_pos.0 - cam.prev.x) / 300.0;
        let dy = (cursor_pos.1 - cam.prev.y) / 300.0;

        cam.smooth_angles = cam
            .smooth_angles
            .lerp(Vector2::new(dx, dy), 1.0 / cam.smoothness);

        cam.angles.x -= dx + cam.smooth_angles.x;
        cam.angles.y -= dy + cam.smooth_angles.y;

        let clamp_val = Rad::from(cam.clamp_y).0;
        cam.angles.y = cam.angles.y.clamp(-clamp_val, clamp_val);

        let x = Rad(cam.angles.y).0.cos() * Rad(cam.angles.x).0.sin(); // x = cos(rad(CY)) * sin(rad(CX))
        let y = Rad(cam.angles.y).0.sin(); // y = sin(rad(CY))
        let z = Rad(cam.angles.y).0.cos() * Rad(cam.angles.x).0.cos(); // z = cos(rad(CY)) * cos(rad(CX))

        cam.forward = Vector3::new(x, y, z).normalize();
    }
    cam.prev = Vector2::new(cursor_pos.0, cursor_pos.1);

    let left = cam.forward.cross(Vector3::unit_y()).normalize();
    let mut eye = trans.position;

    if key_state.is_key_pressed(Key::W) {
        eye += Vector3::from_value(cam.speed).mul_element_wise(cam.forward);
    }

    if key_state.is_key_pressed(Key::A) {
        eye -= Vector3::from_value(cam.speed).mul_element_wise(left);
    }

    if key_state.is_key_pressed(Key::S) {
        eye -= Vector3::from_value(cam.speed).mul_element_wise(cam.forward);
    }

    if key_state.is_key_pressed(Key::D) {
        eye += Vector3::from_value(cam.speed).mul_element_wise(left);
    }

    let at = eye + cam.forward;

    trans.position = eye;

    let projection_matrix = perspective(cam.fov, aspect_ratio, cam.near_clip, cam.far_clip);

    let view_matrix = Matrix4::look_at_rh(
        Point3::from_vec(eye),
        Point3::from_vec(at),
        Vector3::unit_y(),
    );

    CORRECTION_MATRIX * projection_matrix * view_matrix
}

#[rustfmt::skip]
#[allow(unused)]
const CORRECTION_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
