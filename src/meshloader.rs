extern crate stl_io;
extern crate glam;
use glam::vec3a;
use glam::f32::Vec3A;
use stl_io::{IndexedMesh, IndexedTriangle};
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct Mesh {
    pub path: String,
    pub vertices_normals: Vec<Vec3A>,
    pub faces: Vec<IndexedTriangle>,
    pub vertices: Vec<Vec3A>,
}

impl Mesh {
    pub fn new(path: &str, scale: Vec3A) -> Self {
        let mut vertices_normals = Vec::new();
        let mut vertices = Vec::new();

        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        let mesh = stl_io::read_stl(&mut file).unwrap();

        for v in &mesh.vertices {
            let x = v[0] * scale[0];
            let y = v[1] * scale[1];
            let z = v[2] * scale[2];
            vertices.push(vec3a(x,y,z));
        }

        for face in &mesh.faces {
            let n = vec3a(face.normal[0], face.normal[1], face.normal[2]);
            for i in face.vertices {
                let v = &mesh.vertices[i as usize];
                vertices_normals.push(
                    vec3a(scale[0]*v[0], scale[1]*v[1], scale[2]*v[2])
                );
                vertices_normals.push(n);
            }
        }

        Mesh {
            path: String::from(path),
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
    pub fn mutate(&mut self, idx: usize, dir: Vec3A) {
        self.vertices[idx] += dir*0.01;
        self.vertices_normals = vec!();
        for face in &self.faces {
            let mut n = vec3a(face.normal[0], face.normal[1], face.normal[2]);
            let mut triangle_verts = [vec3a(0.0,0.0,0.0);3];
            for i in 0..3 {
                let v = &self.vertices[face.vertices[i] as usize];
                triangle_verts[i] = vec3a(v[0], v[1], v[2]);
            }
            n = (triangle_verts[1]-triangle_verts[0]).cross(triangle_verts[2]-triangle_verts[0]).normalize();
            for i in face.vertices {
                let v = &self.vertices[i as usize];
                self.vertices_normals.push(
                    vec3a(v[0], v[1], v[2])
                );
                self.vertices_normals.push(n);
            }
        }
    }
}
