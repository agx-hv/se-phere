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
    pub fn t_mesh(&self) -> crate::meshloader::Mesh {
        let t_mat = translate(&Matrix4::<f32>::one(), self.pos);
        let mut result = crate::meshloader::Mesh{vertices: vec!()};
        for vertex in &self.mesh.vertices {
            result.vertices.push((t_mat * vertex.extend(1.0)).truncate(3));
        }
        return result;
    }
}
