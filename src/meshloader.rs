extern crate stl_io;
extern crate glm;
use std::fs::OpenOptions;

pub struct Mesh {
    pub vertices: Vec<glm::Vector3<f32>>,
}

impl Mesh {
    pub fn load(&mut self, stl_path: &str) {
        let mut file = OpenOptions::new().read(true).open(stl_path).unwrap();
        let mesh = stl_io::read_stl(&mut file).unwrap();

        for face in mesh.faces {
            for i in face.vertices {
                let v = mesh.vertices[i as usize];
                self.vertices.push(
                    glm::vec3(v[0], v[1], v[2])
                );
            }
        }
    }
}
