extern crate glam;
extern crate gl;
use glam::*;
use crate::meshloader::Mesh;
use crate::camera::PlayerCamera;
use crate::shader::ShaderProgram;

pub struct Player {
    pub vec: Vec3A,
    pub entity: Entity, // inherits Entity struct
    pub camera: PlayerCamera,
    pub radius: f32,
    pub on_ground: bool,
    pub ability: Ability,
}

pub struct Ability {
    pub ground_mut_power: f32,
}

pub struct Entity {
    pub mesh: Mesh,
    pub pos: Vec3A,
    pub vao: u32,
    pub vbo: u32,
    pub color: Vec3A,
    pub bounce: f32,
    pub scale: Vec3A,
}

impl Player {
    pub fn new(stl_path: &str, pos: Vec3A, color: Vec3A, camera: PlayerCamera, bounce: f32) -> Self {
        let e = Entity::new(stl_path,pos,color,bounce);
        Player {
            vec: vec3a(0.0, 0.0, 0.0),
            entity: e,
            camera,
            radius: 0.1,
            on_ground: false,
            ability: Ability{ground_mut_power: 0.01},
        }
    }
    pub fn mv(&mut self, t_vec: Vec3A) { // function to add velocity
        self.vec += t_vec;
        const GRAV_DELTA: f32 = 0.01;
        self.vec += vec3a(0.0, -GRAV_DELTA, 0.0); // gravity as vec3a.y
    }
    pub fn mvhelper(&mut self) { // function to manage velocity - self.vec
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
    pub fn collide(&mut self, other: &Entity) { 
        let (collided, norm, dist) = self.detect_col(other);
        if collided {
            self.entity.pos += dist*norm; // Prevent clipping into collided object 
            self.vec -= self.vec.dot(norm) * norm * (1.0 + self.entity.bounce*other.bounce); // bonuce formula
        }
    }

    pub fn detect_col(&self, other: &Entity) -> (bool, Vec3A, f32) {
        // Performing collision detection logic
        // There must be a more efficient way other than checking ALL mesh faces
        for face in &other.mesh.faces {
            let a = other.mesh.vertices[face.vertices[0]] + other.pos;
            let b = other.mesh.vertices[face.vertices[1]] + other.pos;
            let c = other.mesh.vertices[face.vertices[2]] + other.pos;
            let face_normal = vec3a(face.normal[0],face.normal[1],face.normal[2]).normalize();

            let d = (self.entity.pos-a).project_onto(face_normal).length();

            // Check if plane intersects sphere
            if d <= self.radius {
                let p = self.entity.pos - d*face_normal;
                let n = (b-a).cross(c-a);
                let area = n.length() * 0.5;

                // Barycentric check if sphere-plane intersection point is in triangle
                let f = |x: Vec3A, y:Vec3A, z: Vec3A| -> f32 {
                    let ng = (y-x).cross(z-x);
                    let m = ng.dot(n);
                    if m>=0.0 {
                        return 0.5*ng.length()/area;
                    } else {
                        return -0.5*ng.length()/area;
                    }
                };

                let alpha = f(p,b,c);
                let beta = f(a,p,c);
                let gamma = f(a,b,p);

                let inrange = |x| (x>=0.0 && x<=1.0);
                if inrange(alpha) && inrange(beta) && inrange(gamma) {
                    return (true,face_normal,self.radius-d);
                }
            }
        }

        (false,vec3a(0.0,0.0,0.0),0.0)

    }

}

impl Entity {
    pub fn new(stl_path: &str, pos: Vec3A, color: Vec3A, bounce: f32) -> Self {
        let scale = vec3a(1.0,1.0,1.0);
        let m = Mesh::new(stl_path, scale);
        Entity {
            mesh: m,
            pos,
            vao: 0,
            vbo: 0,
            color,
            bounce,
            scale,
        }
    }
    pub fn set_pos(&mut self, new_pos:Vec3A){
        self.pos = new_pos
    }
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = vec3a(x,y,z);
        let m = Mesh::new(&self.mesh.path as &str , self.scale);
        self.mesh = m;
    }
    pub fn mv(&mut self, t_vec: Vec3A) {
        self.pos += t_vec;
    }
    pub fn set_color(&mut self, color: Vec3A) {
        self.color = color;
    }
    pub unsafe fn gl_init(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        assert_ne!(self.vao,0);
        gl::GenBuffers(1, &mut self.vbo);
        assert_ne!(self.vbo,0);
    }

    pub unsafe fn draw(&mut self, camera: &mut PlayerCamera, lighting_program: &ShaderProgram) {

        let vertices = self.mesh.vertices_flattened();
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
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

        let t_mat = Mat4::from_translation(self.pos.into());
        lighting_program.set_mat4f(b"proj\0",&camera.proj_mat().to_cols_array()[0]);
        lighting_program.set_mat4f(b"view\0",&camera.view_mat().to_cols_array()[0]);
        lighting_program.set_mat4f(b"model\0",&t_mat.to_cols_array()[0]);
        lighting_program.set_vec3f(b"objectColor\0", self.color[0], self.color[1], self.color[2]);

        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.vertices_normals.len() as i32);

    }
    pub fn closest_vertex_index(&mut self, xz: Vec2) -> usize {
        let m = &mut self.mesh;
        let mut min_d = f32::MAX;

        let mut closest_idx = 0 as usize;

        let mut i = 0;
        for v in &m.vertices {
            let d = v.distance(vec3a(xz.x,0.0,xz.y));
            if d < min_d {
                min_d = d;
                closest_idx = i;
            }
            i += 1;
        }
        closest_idx
    }

}
