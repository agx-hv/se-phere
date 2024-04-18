//imports
extern crate glam;
use glam::f32::{Mat4, Vec3A};
use glam::vec3a;
use std::f32::consts::PI;
use crate::Entity;

// Abstract Struct
pub struct CameraBare {
    // For use in camera
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraBare {
    //get a projection matrix
    fn proj_mat(&mut self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.into(),
            self.aspect.into(),
            self.near.into(),
            self.far.into(),
        )
    }
}

// Camera
pub struct PlayerCamera {
    pub player_pos: Vec3A, //players coords
    pub camera_angle: f32, // 0 to 2pi, 0 is behind player
    pub tilt: f32,         // 0 to pi - tilt from ground to bird's eye
    pub radius: f32,       // camera distance away from player
    pub camera_bare: CameraBare,
}

impl PlayerCamera {
    //helper function to create new camera, with defaults
    pub fn new(player_pos: Vec3A, aspect: f32, camera_angle: f32) -> Self {
        PlayerCamera {
            player_pos,
            camera_angle,
            tilt: 0.6,
            radius: 2.0,
            camera_bare: CameraBare {
                fov: PI / 3.0,
                aspect,
                near: 0.01,
                far: 100.0,
            },
        }
    }
    // helper function to update camera to 'new' sphere when respawn
    pub fn update(
        player_pos: Vec3A,
        aspect: f32,
        camera_angle: f32,
        old_cam: PlayerCamera,
    ) -> Self {
        PlayerCamera {
            player_pos,
            camera_angle,
            tilt: old_cam.tilt,
            radius: old_cam.radius,
            camera_bare: CameraBare {
                fov: old_cam.camera_bare.fov,
                aspect,
                near: old_cam.camera_bare.near,
                far: old_cam.camera_bare.far,
            },
        }
    }

    pub fn detect_col(&self, other: &Entity) -> (bool, f32) {
        // Performing collision detection logic
        for face in &other.mesh.faces {
            let a = other.mesh.vertices[face.vertices[0]] + other.pos;
            let b = other.mesh.vertices[face.vertices[1]] + other.pos;
            let c = other.mesh.vertices[face.vertices[2]] + other.pos;
            let face_normal = vec3a(face.normal[0], face.normal[1], face.normal[2]).normalize();

            let d = (a - self.eye()).dot(face_normal);

            // Check if plane intersects sphere
            if d <= 0.0 {
                let p = self.eye() - d * face_normal;
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
                    return (true, -d);
                }
            }
        }

        (false, 0.0)
    }

    pub fn collide(&mut self, ground: &Entity) {
        let (collide_ground, amt) = self.detect_col(ground);
        if collide_ground {
            let hypotenuse = (self.player_pos - self.eye()).length();
            self.tilt += f32::sin(amt);
        }
    }

    //main function to get view matrix
    pub fn view_mat(&mut self) -> Mat4 {
        if self.radius < 0.1 {
            self.radius = 0.1 //prevents zooming in too close
        }
        if self.camera_angle < 0.0 {
            self.camera_angle += 2.0 * PI; // allows for camera to spin horinzontaly constantly around player while preventing int underflow
        }
        else if self.camera_angle > 2.0 * PI{
            self.camera_angle -= 2.0 * PI; // allows for camera to spin horinzontaly constantly around player while preventing int overflow
        }
        self.camera_angle = self.camera_angle % (2.0 * PI);

        if self.tilt > PI / 2.0 {
            self.tilt = PI / 2.0; // clip max tilt to 90deg
        } else if self.tilt < 1e-6 {
            self.tilt = 1e-6; // prevent edge case of completely flat camera
        }

        Mat4::look_at_rh(self.eye().into(), self.player_pos.into(), self.up().into())
    }

    //helper function to get eye vector
    pub fn eye(&self) -> Vec3A {
        vec3a(
            self.radius * f32::sin(self.camera_angle) * f32::cos(self.tilt),
            self.radius * f32::sin(self.tilt),
            self.radius * f32::cos(self.camera_angle) * f32::cos(self.tilt),
        ) + self.player_pos
    }

    //helper function to get up vector
    pub fn up(&self) -> Vec3A {
        Vec3A::normalize(vec3a(
            self.radius * f32::sin(self.camera_angle) * -f32::sin(self.tilt),
            self.radius * f32::cos(self.tilt),
            self.radius * f32::cos(self.camera_angle) * -f32::sin(self.tilt),
        ))
    }

    //gets projection matrix
    pub fn proj_mat(&mut self) -> Mat4 {
        self.camera_bare.proj_mat()
    }
}
