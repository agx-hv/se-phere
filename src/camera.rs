extern crate glm;
use glm::*;
use glm::ext::*;

pub struct Camera {
    pub eye: Vector3<f32>,
    pub center: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn pv_mat(&mut self) -> Matrix4<f32> {
        let v_mat = look_at::<f32>(self.eye, self.center, self.up);
        let p_mat = perspective::<f32>(self.fov, self.aspect, self.near, self.far);
        p_mat * v_mat
    }
}
