extern crate glam;
use glam::*;
use crate::meshloader::Mesh;

pub struct Player {
    pub vec: Vec3A,
    pub entity: Entity,
}

pub struct Entity {
    pub mesh: Mesh,
    pub pos: Vec3A,
}

impl Player {
    pub fn new(stl_path: &str, pos: Vec3A) -> Self {
        let e = Entity::new(stl_path,pos);
        Player {
            vec: vec3a(0.0, 0.0, 0.0),
            entity: e,
        }
    }
    pub fn mv(&mut self, t_vec: Vec3A) {
        self.vec += t_vec;
    }
    pub fn mvhelper(&mut self) {
        self.entity.pos += self.vec;

        const VEC_DELTA: f32 = 0.95;
        self.vec *= VEC_DELTA;
    }
    pub fn pos(&self) -> Vec3A {
        self.entity.pos
    }
    pub fn mesh(&self) -> &Mesh {
        &self.entity.mesh
    }
}

impl Entity {
    pub fn new(stl_path: &str, pos: Vec3A) -> Self {
        let m = Mesh::new(stl_path);
        Entity {
            mesh: m,
            pos: pos,
        }
    }
    pub fn mv(&mut self, t_vec: Vec3A) {
        self.pos += t_vec;
    }
    pub unsafe fn render(&self) {

    }
}
