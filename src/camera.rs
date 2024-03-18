extern crate glm;
use glm::*;
use glm::ext::*;
use crate::utils::*;

pub struct Camera {
    pub eye: Vector3<f32>,
    pub center: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    pub vec: Vector3<f32>,
}

impl Camera {
    pub fn pv_mat(&mut self) -> Matrix4<f32> {
        let v_mat = look_at::<f32>(self.eye, self.center, self.up);
        let p_mat = perspective::<f32>(self.fov, self.aspect, self.near, self.far);
        p_mat * v_mat
    }
    pub fn mvhelper(&mut self, p_pos: Vector3<f32>, p_vec: Vector3<f32>) {
        xyz_plus_xyz(self.vec,p_vec);

        const CAM_DELTA: f32 = 0.01;
        self.eye.x += self.vec.x * CAM_DELTA;
        self.center.x += self.center.x * CAM_DELTA;
        self.eye.y += self.vec.y * CAM_DELTA;
        self.eye.z += self.vec.z * CAM_DELTA;
        self.center.z += self.center.z * CAM_DELTA;

        self.vec.x *= 1.0 - CAM_DELTA;
        self.vec.y *= 1.0 - CAM_DELTA;
        self.vec.z *= 1.0 - CAM_DELTA;
        
    }
}
