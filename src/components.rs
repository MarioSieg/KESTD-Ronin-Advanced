use super::math::*;
use super::resources::{material::Material, mesh::Mesh};
use std::sync::Arc;

#[derive(Clone)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    #[inline]
    pub fn calculate_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }
}

#[derive(Clone)]
pub struct MeshRenderer {
    pub mesh: Arc<Mesh>,
    pub material: Arc<Material>,
}

#[derive(Clone)]
pub struct Camera {
    pub fov: Deg<f32>,
    pub near_clip: f32,
    pub far_clip: f32,
    pub clamp_y: f32,
    pub smoothness: f32,
    pub speed: f32,
    pub prev: Vector2<f32>,
    pub angles: Vector2<f32>,
    pub smooth_angles: Vector2<f32>,
    pub forward: Vector3<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            fov: Deg(75.0),
            near_clip: 0.1,
            far_clip: 100.0,
            clamp_y: 80.0,
            smoothness: 1.5,
            speed: 0.01,
            prev: Vector2::zero(),
            angles: Vector2::zero(),
            smooth_angles: Vector2::zero(),
            forward: Vector3::zero(),
        }
    }
}
