extern crate stl_io;
extern crate glam;
use glam::{vec3a,vec2};
use glam::f32::{Vec3A,Vec2,Mat3A};
use stl_io::IndexedTriangle;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct Mesh {
    pub path: String,
    pub vertices_normals: Vec<Vec3A>,
    pub faces: Vec<IndexedTriangle>,
    pub vertices: Vec<Vec3A>,
    pub tex_coords: Vec<Vec2>, // New field for texture coordinates
}

impl Mesh {
    pub fn new(path: &str, scale: Vec3A) -> Self {
        let mut vertices_normals = Vec::new();
        let mut vertices = Vec::new();
        let mut tex_coords = Vec::new(); // Initialize texture coordinates vector

        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        let mesh = stl_io::read_stl(&mut file).unwrap();

        for v in &mesh.vertices {
            let x = v[0] * scale[0];
            let y = v[1] * scale[1];
            let z = v[2] * scale[2];
            vertices.push(vec3a(x,y,z));
        }

        // Calculate texture coordinates based on vertex positions
        for v in &vertices {
            let u = v.x * 100.0; // Example: Using x-coordinate as texture U coordinate
            let v = v.z * 100.0; // Example: Using y-coordinate as texture V coordinate
            tex_coords.push(vec2(u, v));
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
            tex_coords, // Assign texture coordinates to the struct field
        }
    }
    pub fn vertices_flattened(&self) -> Vec<f32> {
        let mut v = vec!();
        for vertex in &self.vertices_normals {
            v.extend_from_slice(&mut vertex.to_array().as_slice());
        }
        v
    }
    pub fn mutate(&mut self, idx: usize, dir: Vec3A, amount: f32) {
        self.vertices[idx] += dir*amount;
        self.vertices_normals = vec!();
        for face in &mut self.faces {
            let n: Vec3A;
            let mut triangle_verts = [vec3a(0.0,0.0,0.0);3];
            for i in 0..3 {
                let v = &self.vertices[face.vertices[i] as usize];
                triangle_verts[i] = vec3a(v[0], v[1], v[2]);
            }
            n = (triangle_verts[1]-triangle_verts[0]).cross(triangle_verts[2]-triangle_verts[0]).normalize();
            face.normal = stl_io::Vector::new([n.x,n.y,n.z]);
            for i in face.vertices {
                let v = &self.vertices[i as usize];
                self.vertices_normals.push(
                    vec3a(v[0], v[1], v[2])
                );
                self.vertices_normals.push(n);
            }
        }
    }
    pub fn set_vertex_height(&mut self, idx: usize, height: f32) {
        let dir = vec3a(0.0,1.0,0.0);
        self.vertices[idx] = dir*height;
        self.vertices_normals = vec!();
        for face in &mut self.faces {
            let n: Vec3A;
            let mut triangle_verts = [vec3a(0.0,0.0,0.0);3];
            for i in 0..3 {
                let v = &self.vertices[face.vertices[i] as usize];
                triangle_verts[i] = vec3a(v[0], v[1], v[2]);
            }
            n = (triangle_verts[1]-triangle_verts[0]).cross(triangle_verts[2]-triangle_verts[0]).normalize();
            face.normal = stl_io::Vector::new([n.x,n.y,n.z]);
            for i in face.vertices {
                let v = &self.vertices[i as usize];
                self.vertices_normals.push(
                    vec3a(v[0], v[1], v[2])
                );
                self.vertices_normals.push(n);
            }
        }
    }
    pub fn rotate_y(&mut self, theta: f32) {
        let ry = Mat3A::from_rotation_y(theta);
        for mut v in &self.vertices {
            v = &(ry**v);
        }
        self.vertices_normals = vec!();
        for face in &mut self.faces {
            let n: Vec3A;
            let mut triangle_verts = [vec3a(0.0,0.0,0.0);3];
            for i in 0..3 {
                let v = &self.vertices[face.vertices[i] as usize];
                triangle_verts[i] = ry*vec3a(v[0], v[1], v[2]);
            }
            n = (triangle_verts[1]-triangle_verts[0]).cross(triangle_verts[2]-triangle_verts[0]).normalize();
            face.normal = stl_io::Vector::new([n.x,n.y,n.z]);
            for i in face.vertices {
                let v = &self.vertices[i as usize];
                self.vertices_normals.push(
                    ry*vec3a(v[0], v[1], v[2])
                );
                self.vertices_normals.push(n);
            }
        }
    }
}
