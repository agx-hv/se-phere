extern crate glam;
extern crate gl;
use glam::*;
use crate::meshloader::Mesh;
use crate::camera::PlayerCamera;
use crate::shader::ShaderProgram;

pub struct Player {
    pub vec: Vec3A,
    pub entity: Entity,
}

pub struct Entity {
    pub mesh: Mesh,
    pub pos: Vec3A,
    pub vao: u32,
    pub color: Vec3A,
}

impl Player {
    pub fn new(stl_path: &str, pos: Vec3A, color: Vec3A) -> Self {
        let e = Entity::new(stl_path,pos,color);
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
    pub fn new(stl_path: &str, pos: Vec3A, color: Vec3A) -> Self {
        let m = Mesh::new(stl_path);
        Entity {
            mesh: m,
            pos: pos,
            vao: 0,
            color: color,
        }
    }
    pub fn mv(&mut self, t_vec: Vec3A) {
        self.pos += t_vec;
    }
    pub fn set_color(&mut self, color: Vec3A) {
        self.color = color;
    }
    pub unsafe fn gl_init(&mut self) {
        let mut vbo = 0u32;

        let vertices = self.mesh.vertices_flattened();

        gl::GenVertexArrays(1, &mut self.vao);
        assert_ne!(self.vao,0);
        gl::BindVertexArray(self.vao);

        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo,0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(gl::ARRAY_BUFFER, 
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6*std::mem::size_of::<f32>()).try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6*std::mem::size_of::<f32>()).try_into().unwrap(),
            (3*std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
    }

    pub unsafe fn draw(&self, camera: &mut PlayerCamera, lighting_program: &ShaderProgram) {
            let t_mat = Mat4::from_translation(self.pos.into());
            lighting_program.set_mat4f(b"proj\0",&camera.proj_mat().to_cols_array()[0]);
            lighting_program.set_mat4f(b"view\0",&camera.view_mat().to_cols_array()[0]);
            lighting_program.set_mat4f(b"model\0",&t_mat.to_cols_array()[0]);
            lighting_program.set_vec3f(b"objectColor\0", self.color[0], self.color[1], self.color[2]);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.vertices.len() as i32);
    }
    pub fn mutate(&mut self, closest: Vec3A, direction: Vec3A) {
        let m = &mut self.mesh;
    }

    pub fn detect_col(&self, other: &Entity) -> bool {
        // Perform collision detection logic here

        // if self.pos.distance(other.pos) < 0.5 { // this works for smooth detection - only center entity
        //     for vert in &other.mesh.vertices {
        //         if self.pos.distance(*vert) < 0.1 {
        //             return true; // Collision detected
        //         }
        //     }
        // }false

        // if self.pos.distance(other.pos) < 0.25{ // this works for one on one detection - laggy
        //     for vertex1 in &self.mesh.vertices {
        //         for vertex2 in &other.mesh.vertices {
        //             if vertex1.x == vertex2.x && vertex1.y == vertex2.y && vertex1.z == vertex2.z {
        //                 return true; // Collision detected
        //             }
        //         }
        //     }
        // }false

        for face in &other.mesh.faces {
            let a = other.mesh.vertices[face.vertices[0]];
            let b = other.mesh.vertices[face.vertices[1]];
            let c = other.mesh.vertices[face.vertices[2]];
            let face_normal = vec3a(face.normal[0],face.normal[1],face.normal[2]);
            let face_mid = (a+b+c)/3.0;

            if face_mid.distance(self.pos) < 0.3{
                if a.project_onto(face_normal).length_squared() < 0.01 || b.project_onto(face_normal).length_squared() < 0.01 ||
                c.project_onto(face_normal).length_squared() < 0.01 { return true }
            }
                
        }false

        // if self.pos.distance(other.pos) < 0.5{ // this works for one on one detection - not rly laggy
        //     for vertex2 in &other.mesh.vertices {
        //     if f32::abs(self.pos.x - vertex2.x)<0.2 &&
        //     f32::abs(self.pos.z - vertex2.z)<0.2{
        //             return true; // Collision detected
        //         }
        //     }
        // }false

        // let collision_threshold = 0.2; // this works for smooth detection - only center entity - lights everything
        // for vert in &other.mesh.vertices {
        //     let distance = self.pos.distance(*vert);
        //     if distance < collision_threshold {
        //         return true;
        //     }
        // }false
    }


}
