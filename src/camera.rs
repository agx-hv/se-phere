extern crate glam;
use glam::vec3a;
use glam::f32::{Mat4, Vec3A};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3A,
    pub center: Vec3A,
    pub up: Vec3A,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    pub vec: Vec3A,
}

impl Camera {
    pub fn view_mat(&mut self) -> Mat4 {
        Mat4::look_at_rh(self.eye.into(), self.center.into(), self.up.into())
    }
    pub fn proj_mat(&mut self) -> Mat4 {
        Mat4::perspective_rh(self.fov.into(), self.aspect.into(), self.near.into(), self.far.into())
    }

    pub fn mvhelper(&mut self, p_pos: Vec3A, p_vec: Vec3A) {
        self.vec += p_vec;

        const CAM_DELTA: f32 = 0.01;
        self.eye.x += self.vec.x * CAM_DELTA;
        self.center.x += self.center.x * CAM_DELTA;
        self.eye.y += self.vec.y * CAM_DELTA;
        self.eye.z += self.vec.z * CAM_DELTA;
        self.center.z += self.center.z * CAM_DELTA;

        self.vec.x *= 1.0 - CAM_DELTA;
        self.vec.y *= 1.0 - CAM_DELTA;
        self.vec.z *= 1.0 - CAM_DELTA;
        
    }
}


pub struct PlayerCamera {
    pub player_pos: Vec3A, //players coords
    pub camera_angle: f32, // 0 to 2pi, 0 is behind player
    pub tilt: f32, // 0 to pi - tilt from ground to bird's eye
    pub radius: f32, // camera distance away from player
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl PlayerCamera{
    pub fn view_mat(&mut self) -> Mat4 {
        if self.camera_angle<0.0{
            self.camera_angle += 2.0*PI; //alows for camera to spin horinzontaly constantly around player while preventing int overflow
        }
        self.camera_angle=self.camera_angle%(2.0*PI);
        if self.tilt > PI/2.0{
            self.tilt = PI/2.0; //clip max tilt to 90deg
        }
        else if self.tilt < 1e-6{
            self.tilt = 1e-6; //prevent edge case of completely flat camera
        }
        let eye: Vec3A = vec3a(
            self.radius*f32::sin(self.camera_angle)*f32::cos(self.tilt),
            self.radius*f32::sin(self.tilt),
            self.radius*f32::cos(self.camera_angle)*f32::cos(self.tilt),
            )
            +self.player_pos;

        let up: Vec3A = Vec3A::normalize(vec3a(
            self.radius*f32::sin(self.camera_angle)*-f32::sin(self.tilt),
            self.radius*f32::sin(self.tilt),
            self.radius*f32::cos(self.camera_angle)*-f32::sin(self.tilt),
            )
            );

        Mat4::look_at_rh(eye.into(), self.player_pos.into(), up.into())
        
    }
    pub fn proj_mat(&mut self) -> Mat4 {
        Mat4::perspective_rh(self.fov.into(), self.aspect.into(), self.near.into(), self.far.into())
    }
}
