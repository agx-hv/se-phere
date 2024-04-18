extern crate stl_io;
extern crate glam;
use glam::vec3a;
use glam::f32::{Vec3A,Mat3A};
use stl_io::IndexedTriangle;
use std::fs::OpenOptions;

// Mesh struct to store vertex and face and normals information
#[derive(Debug)]
pub struct Mesh {
    pub path: String,
    pub faces: Vec<IndexedTriangle>,
    pub vertices: Vec<Vec3A>,
    pub vertices_normals_tex: Vec<Vec3A>, // New field for texture coordinates
}

// Mesh methods
impl Mesh {
    
    // Mesh constructor that takes stl path and scale as arguments
    // Parses the stl file to extract vertex and face information and stores the scaled mesh into
    // the Mesh struct. 
    // vertices_normals_tex is used for shading.
    pub fn new(path: &str, scale: Vec3A) -> Self {
        let mut vertices = Vec::new();
        let mut vertices_normals_tex = Vec::new(); // Initialize texture coordinates vector

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
                vertices_normals_tex.push(
                    vec3a(scale[0]*v[0], scale[1]*v[1], scale[2]*v[2])
                );
                vertices_normals_tex.push(n);
                // Calculate texture coordinates based on vertex positions
                let u = v[0]; // Example: Using x-coordinate as texture U coordinate
                let v = v[2]; // Example: Using y-coordinate as texture V coordinate
                vertices_normals_tex.push(vec3a(u, v, 0.0));
            }
        }
        

        Mesh {
            path: String::from(path),
            faces: mesh.faces,
            vertices,
            vertices_normals_tex, // Assign texture coordinates to the struct field
        }
    }

    // Method to flatten vertices_normals_tex into 1-dimension to be sent to shader
    pub fn vertices_flattened(&self) -> Vec<f32> {
        let mut v = vec!();
        for vertex in &self.vertices_normals_tex {
            v.extend_from_slice(&mut vertex.to_array().as_slice());
        }
        v
    }

    // Method to mutate a single vertex of the mesh by some direction and amount
    pub fn mutate(&mut self, idx: usize, dir: Vec3A, amount: f32) {
        self.vertices[idx] += dir*amount;
        self.vertices_normals_tex = vec!();
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

                self.vertices_normals_tex.push(
                    vec3a(v[0], v[1], v[2])
                );
                self.vertices_normals_tex.push(n);
                // Calculate texture coordinates based on vertex positions
                let u = v[0]; // Example: Using x-coordinate as texture U coordinate
                let v = v[2]; // Example: Using y-coordinate as texture V coordinate
                self.vertices_normals_tex.push(vec3a(u, v, 0.0));
            }
        }
    }

    // Method to rotate the mesh about y-axis to allow animated spinning entities to be rendered
    pub fn rotate_y(&mut self, theta: f32) {
        let ry = Mat3A::from_rotation_y(theta);
        for mut v in &self.vertices {
            v = &(ry**v);
        }

        self.vertices_normals_tex = vec!();
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

                self.vertices_normals_tex.push(
                    ry*vec3a(v[0], v[1], v[2])
                );
                self.vertices_normals_tex.push(n);
                // Calculate texture coordinates based on vertex positions
                let u = v[0]; // Example: Using x-coordinate as texture U coordinate
                let v = v[2]; // Example: Using y-coordinate as texture V coordinate
                self.vertices_normals_tex.push(vec3a(u, v, 0.0));
            }
        }
    }
}
