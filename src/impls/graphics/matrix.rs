use cgmath::*;

#[rustfmt::skip]
#[allow(unused)]
const CORRECTION_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub fn generate_matrix(aspect_ratio: f32) -> Matrix4<f32> {
    let mx_projection = perspective(Deg(45f32), aspect_ratio, 1.0, 10.0);
    let mx_view = Matrix4::look_at_rh(
        Point3::new(1.5, 0.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::unit_y(),
    );
    CORRECTION_MATRIX * mx_projection * mx_view
}
