extern crate glm;
use glm::*;
use glm::ext::*;
use crate::num_traits::One;
use crate::utils::*;

pub struct Player{
    pub mesh: crate::meshloader::Mesh,
    pub pos: Vector3<f32>,
    pub vec: Vector3<f32>,
}

impl Player {
    pub fn mv(&mut self, t_vec: Vector3<f32>) {
        let t_mat = translate(&Matrix4::<f32>::one(),t_vec);
        self.pos = (t_mat * self.pos.extend(1.0)).truncate(3);
        self.vec.x += t_vec.x;
        self.vec.y += t_vec.y;
        self.vec.z += t_vec.z;
    }
    pub fn mvhelper(&mut self) {
        self.pos.x += self.vec.x;
        self.pos.y += self.vec.y;
        self.pos.z += self.vec.z;

        const VEC_DELTA: f32 = 0.95;
        self.vec.x *= VEC_DELTA; // You can adjust the factor (0.9) to control the speed of reduction
        self.vec.y *= VEC_DELTA;
        self.vec.z *= VEC_DELTA;
    }
}