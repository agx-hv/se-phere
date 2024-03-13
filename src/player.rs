extern crate glm;
use glm::*;
use glm::ext::*;
use crate::num_traits::One;

pub struct Player{
    pub mesh: crate::meshloader::Mesh,
    pub pos: Vector3<f32>,
}

impl Player {
    pub fn mv(&mut self, t_vec: Vector3<f32>) {
        let t_mat = translate(&Matrix4::<f32>::one(),t_vec);
        self.pos = (t_mat * self.pos.extend(1.0)).truncate(3);
    }
}
