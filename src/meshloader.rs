extern crate stl_io;
extern crate glam;
use glam::vec3a;
use glam::f32::Vec3A;
use std::fs::OpenOptions;

pub struct Mesh {
    pub vertices: Vec<Vec3A>,
}

impl Mesh {
    pub fn load(&mut self, stl_path: &str) {
        let mut file = OpenOptions::new().read(true).open(stl_path).unwrap();
        let mesh = stl_io::read_stl(&mut file).unwrap();

        for face in mesh.faces {
            let n = face.normal;
            for i in face.vertices {
                let v = mesh.vertices[i as usize];
                self.vertices.push(
                    vec3a(v[0], v[1], v[2])
                );
                self.vertices.push(
                    vec3a(n[0], n[1], n[2])
                );
            }
        }
    }
}
