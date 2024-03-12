extern crate stl_io;
use std::fs::OpenOptions;

pub struct Mesh {
    pub vertices: Vec<f32>,
}

impl Mesh {
    pub fn load(&mut self, stl_path: &str) {
        let mut file = OpenOptions::new().read(true).open(stl_path).unwrap();
        let mesh = stl_io::read_stl(&mut file).unwrap();

        for face in mesh.faces {
            for i in face.vertices {
                let v = mesh.vertices[i as usize];
                self.vertices.push(v[0]);
                self.vertices.push(v[1]);
                self.vertices.push(v[2]);
            }
        }
    }
}
