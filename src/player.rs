extern crate glam;
use glam::*;
use crate::num_traits::One;

pub struct Player{
    pub mesh: crate::meshloader::Mesh,
    pub pos: Vec3A,
    pub vec: Vec3A,
}

impl Player {
    pub fn mv(&mut self, t_vec: Vec3A) {
        self.vec += t_vec;
    }
    pub fn mvhelper(&mut self) {
        self.pos += self.vec;

        const VEC_DELTA: f32 = 0.95;
        self.vec *= VEC_DELTA;
    }
}
