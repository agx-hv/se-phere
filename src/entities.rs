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
            lighting_program.setMat4f(b"proj\0",&camera.proj_mat().to_cols_array()[0]);
            lighting_program.setMat4f(b"view\0",&camera.view_mat().to_cols_array()[0]);
            lighting_program.setMat4f(b"model\0",&t_mat.to_cols_array()[0]);
            lighting_program.setVec3f(b"objectColor\0", self.color[0], self.color[1], self.color[2]);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.vertices.len() as i32);
    }
}
