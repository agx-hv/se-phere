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
    pub fn mvhelper(&mut self, p_pos: Vector3<f32>, p_vec: Vector3<f32>) {
        const vec_delta: f32 = 0.01;
        self.center.x += ((p_pos.x-self.center.x))*vec_delta;
        self.center.y = p_pos.y;
        self.center.x += ((p_pos.x-self.center.x))*vec_delta;

        self.eye.x += ((p_vec.x-self.eye.x))*vec_delta;
        self.eye.z += ((p_vec.z-self.eye.z))*vec_delta;
        
    }
}
