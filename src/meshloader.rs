extern crate stl_io;
extern crate glam;
use glam::vec3a;
use glam::f32::Vec3A;
use stl_io::{IndexedMesh, IndexedTriangle};
use std::fs::OpenOptions;

pub struct Mesh {
    pub vertices_normals: Vec<Vec3A>,
    pub faces: Vec<IndexedTriangle>,
    pub vertices: Vec<Vec3A>,
}

impl Mesh {
    pub fn new(stl_path: &str) -> Self {
        let mut vertices_normals = Vec::new();
        let mut vertices = Vec::new();

        let mut file = OpenOptions::new().read(true).open(stl_path).unwrap();
        let mesh = stl_io::read_stl(&mut file).unwrap();

        for v in &mesh.vertices {
            vertices.push(vec3a(v[0],v[1],v[2]));
        }

        for face in &mesh.faces {
            let n = face.normal;
            for i in face.vertices {
                let v = &mesh.vertices[i as usize];
                vertices_normals.push(
                    vec3a(v[0], v[1], v[2])
                );
                vertices_normals.push(
                    vec3a(n[0], n[1], n[2])
                );
            }
        }

        Mesh {
            vertices_normals,
            faces: mesh.faces,
            vertices,
        }
    }
    pub fn vertices_flattened(&self) -> Vec<f32> {
        let mut v = vec!();
        for vertex in &self.vertices_normals {
            v.extend_from_slice(&mut vertex.to_array().as_slice());
        }
        v
    }
}
