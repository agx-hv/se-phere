extern crate gl;
extern crate glam;
use crate::camera::PlayerCamera;
use crate::meshloader::Mesh;
use crate::shader::ShaderProgram;
use gl::types::{GLint, GLuint};
use glam::*;

// Player abilities (to be expanded in future)
pub struct Ability {
    pub ground_mut_power: f32,  // Affects how much a player can mutate the ground mesh
}

// Player struct for the main player
pub struct Player {
    pub vec: Vec3A,
    pub entity: Entity, // inherits Entity struct
    pub camera: PlayerCamera,
    pub radius: f32,
    pub on_ground: bool,
    pub ability: Ability,
    pub player_id: u8,
}

// Player methods
impl Player {

    // Player constructor
    pub fn new(
        stl_path: &str,
        pos: Vec3A,
        color: Vec3A,
        camera: PlayerCamera,
        bounce: f32,
        player_id: u8,
    ) -> Self {
        let e = Entity::new(stl_path, pos, color, bounce);
        Player {
            vec: vec3a(0.0, 0.0, 0.0),
            entity: e,
            camera,
            radius: 0.1,
            on_ground: false,
            ability: Ability {
                ground_mut_power: 0.08,
            },
            player_id,
        }
    }

    // Method to move player
    pub fn mv(&mut self, t_vec: Vec3A) {
        // function to add velocity
        if self.on_ground {
            self.vec += t_vec;
        }
        const GRAV_DELTA: f32 = 0.01;
        self.vec += vec3a(0.0, -GRAV_DELTA, 0.0); // gravity as vec3a.y
    }

    // Helper function for drag and friction
    pub fn mvhelper(&mut self) {
        // function to manage velocity - self.vec
        self.entity.pos += self.vec;
        let mut vec_delta: f32 = 0.95;
        if !self.on_ground {
            vec_delta = 0.99;
        }
        self.vec *= vec_delta;
    }

    // Getter for position
    pub fn pos(&self) -> Vec3A {
        self.entity.pos
    }

    // Getter for mesh
    pub fn mesh(&self) -> &Mesh {
        &self.entity.mesh
    }

    // Collision detection for sphere-sphere intersection
    pub fn detect_col_sphere(&self, other: &Entity) -> (bool, Vec3A, f32) {
        // Performing collision detection logic
        let intersection_amt = self.pos().distance(other.pos) - 0.2;
        if intersection_amt > 0.0 {
            let n = (self.pos() - other.pos).normalize();
            return (true, n, intersection_amt);
        }
        (false, vec3a(0.0, 0.0, 0.0), 0.0)
    }

    // Collision detection for sphere-mesh intersection
    pub fn detect_col(&self, other: &Entity) -> (bool, Vec3A, f32) {
        // Performing collision detection logic
        for face in &other.mesh.faces {
            let a = other.mesh.vertices[face.vertices[0]] + other.pos;
            let b = other.mesh.vertices[face.vertices[1]] + other.pos;
            let c = other.mesh.vertices[face.vertices[2]] + other.pos;
            let face_normal = vec3a(face.normal[0], face.normal[1], face.normal[2]).normalize();

            let d = (self.entity.pos - a).project_onto(face_normal).length();

            // Check if plane intersects sphere
            if d <= self.radius {
                let p = self.entity.pos - d * face_normal;
                let n = (b - a).cross(c - a);
                let area = n.length() * 0.5;

                // Barycentric check if sphere-plane intersection point is in triangle
                let f = |x: Vec3A, y: Vec3A, z: Vec3A| -> f32 {
                    let ng = (y - x).cross(z - x);
                    let m = ng.dot(n);
                    if m >= 0.0 {
                        return 0.5 * ng.length() / area;
                    } else {
                        return -0.5 * ng.length() / area;
                    }
                };

                let alpha = f(p, b, c);
                let beta = f(a, p, c);
                let gamma = f(a, b, p);

                // Closure to check if a f32 is in between 0.0 and 1.0
                let inrange = |x| (x >= 0.0 && x <= 1.0); 
                if inrange(alpha) && inrange(beta) && inrange(gamma) {
                    return (true, face_normal, self.radius - d);
                }
            }
        }

        (false, vec3a(0.0, 0.0, 0.0), 0.0)
    }

    // Collision behaviour for sphere-sphere collision
    pub fn collide_sphere(&mut self, other: &Entity) {
        let (collided, norm, dist) = self.detect_col_sphere(other);
        if collided {
            self.entity.pos += dist * norm; // Prevent clipping into collided object
            self.camera.player_pos += dist * norm; // Apply same translation to camera
            self.vec -= self.vec.dot(norm) * norm * (1.0 + self.entity.bounce * other.bounce); // bonuce formula
        }
    }

    // Collision behaviour for sphere-mesh collision
    pub fn collide(&mut self, other: &Entity) {
        let (collided, norm, dist) = self.detect_col(other);
        if collided {
            self.entity.pos += dist * norm; // Prevent clipping into collided object
            self.camera.player_pos += dist * norm; // Apply same translation to camera
            self.vec -= self.vec.dot(norm) * norm * (1.0 + self.entity.bounce * other.bounce); // bonuce formula
        }
    }
    
    // Method that returns the position command to be sent over network
    pub fn pos_cmd(&self) -> [u8; 14] {
        let mut result = vec![0x02];
        result.extend_from_slice(&[self.player_id]);
        result.extend_from_slice(&self.pos().x.to_be_bytes());
        result.extend_from_slice(&self.pos().y.to_be_bytes());
        result.extend_from_slice(&self.pos().z.to_be_bytes());
        //dbg!(&result);
        result.as_slice().try_into().expect("Not 14 bytes long")
    }
}


// Entity struct for all entities in game
#[derive(Debug)]
pub struct Entity {
    pub mesh: Mesh,
    pub pos: Vec3A,
    pub vao: u32,
    pub vbo: u32,
    pub color: Vec3A,
    pub reflectance: f32,
    pub bounce: f32,
    pub scale: Vec3A,
    pub texture_id0: GLuint,
    pub texture_id1: GLuint,
}

// Entity methods
impl Entity {
    // Entity constructor
    pub fn new(stl_path: &str, pos: Vec3A, color: Vec3A, bounce: f32) -> Self {
        let scale = vec3a(1.0, 1.0, 1.0);
        let m = Mesh::new(stl_path, scale);
        Entity {
            mesh: m,
            pos,
            vao: 0,
            vbo: 0,
            color,
            reflectance: 1.0,
            bounce,
            scale,
            texture_id0: 0,
            texture_id1: 0,
        }
    }

    // Setter for entity position
    pub fn set_pos(&mut self, new_pos: Vec3A) {
        self.pos = new_pos
    }
    
    // Method to scale the mesh of an entity
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = vec3a(x, y, z);
        let m = Mesh::new(&self.mesh.path as &str, self.scale);
        self.mesh = m;
    }

    // Method to move an entity
    pub fn mv(&mut self, t_vec: Vec3A) {
        self.pos += t_vec;
    }

    // Setter for entity color
    pub fn set_color(&mut self, color: Vec3A) {
        self.color = color;
    }

    // Method to initialize entity VAO and VBO, as well as loading the textures
    pub unsafe fn gl_init(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        assert_ne!(self.vao, 0);
        gl::GenBuffers(1, &mut self.vbo);
        assert_ne!(self.vbo, 0);

        // textures
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::MIRRORED_REPEAT as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::ActiveTexture(gl::TEXTURE0);
        let texture0 = image::open("assets/textures/white.jpg")
            .expect("Failed to load texture image")
            .flipv()
            .to_rgb8();
        let mut texture_id0 = 0;
        let (width, height) = texture0.dimensions();
        gl::GenTextures(1, &mut texture_id0);
        gl::BindTexture(gl::TEXTURE_2D, texture_id0);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as GLint,
            height as GLint,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture0.as_ptr() as *const _,
        );
        let texture1 = image::open("assets/textures/dirt.jpg")
            .expect("Failed to load texture image")
            .flipv()
            .to_rgb8();
        let mut texture_id1 = 0;
        let (width, height) = texture1.dimensions();
        gl::GenTextures(1, &mut texture_id1);
        gl::BindTexture(gl::TEXTURE_2D, texture_id1);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as GLint,
            height as GLint,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture1.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        self.texture_id0 = texture_id0;
        self.texture_id1 = texture_id1;
    }

    // Method to draw the entity on some player camera using a specified shader
    pub unsafe fn draw(&mut self,
                       camera: &mut PlayerCamera,
                       lighting_program: &ShaderProgram,
                       tex_on: bool) {

        let vertices = self.mesh.vertices_flattened();
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (9 * std::mem::size_of::<f32>()).try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (9 * std::mem::size_of::<f32>()).try_into().unwrap(),
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            3,
            gl::FLOAT,
            gl::FALSE,
            (9 * std::mem::size_of::<f32>()).try_into().unwrap(),
            (6 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(2);

        let t_mat = Mat4::from_translation(self.pos.into());
        lighting_program.set_mat4f(b"proj\0", &camera.proj_mat().to_cols_array()[0]);
        lighting_program.set_mat4f(b"view\0", &camera.view_mat().to_cols_array()[0]);
        lighting_program.set_mat4f(b"model\0", &t_mat.to_cols_array()[0]);
        let object_colour = self.color * self.reflectance;
        lighting_program.set_vec3f(
            b"objectColor\0",
            object_colour[0],
            object_colour[1],
            object_colour[2],
        );

        if tex_on {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id1);
        } else {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id0);
        }
        gl::BindVertexArray(self.vao);
        gl::Uniform1i(self.texture_id1 as GLint, 0);
        gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.vertices_normals_tex.len() as i32);
    }

    // Method to retrieve the closest vertex index at location (x, 0.0, z)
    // Used for ground mutation based on raycasted mouse coordinates
    pub fn closest_vertex_index(&mut self, xz: Vec2) -> usize {
        let m = &mut self.mesh;
        let mut min_d = f32::MAX;

        let mut closest_idx = 0 as usize;

        let mut i = 0;
        for v in &m.vertices {
            let d = v.distance(vec3a(xz.x, 0.0, xz.y));
            if d < min_d {
                min_d = d;
                closest_idx = i;
            }
            i += 1;
        }
        closest_idx
    }
}
