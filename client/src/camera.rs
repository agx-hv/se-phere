extern crate glam;
use glam::vec3a;
use glam::f32::{Mat4, Vec3A};
use std::f32::consts::PI;

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
    pub fn new(player_pos: Vec3A, aspect: f32, camera_angle: f32)-> Self {
        PlayerCamera{
            player_pos,
            camera_angle,
            tilt: 0.6,
            radius: 2.0,
            fov: PI/3.0,
            aspect,
            near: 0.01,
            far: 100.0,
        }
    }

    pub fn update(player_pos: Vec3A, aspect: f32, camera_angle: f32, old_cam: PlayerCamera)-> Self {
        PlayerCamera{
            player_pos,
            camera_angle,
            tilt: old_cam.tilt,
            radius: old_cam.radius,
            fov: old_cam.fov,
            aspect,
            near: old_cam.near,
            far: old_cam.far,
        }
    }

    pub fn view_mat(&mut self) -> Mat4 {
        if self.radius<0.1{
            self.radius = 0.1
        }
        if self.camera_angle<0.0{
            self.camera_angle += 2.0*PI; // allows for camera to spin horinzontaly constantly around player while preventing int overflow
        }
        self.camera_angle=self.camera_angle%(2.0*PI);
        if self.tilt > PI/2.0{
            self.tilt = PI/2.0; // clip max tilt to 90deg
        }
        else if self.tilt < 1e-6{
            self.tilt = 1e-6; // prevent edge case of completely flat camera
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
    pub fn eye(&self) -> Vec3A {
        vec3a(
            self.radius*f32::sin(self.camera_angle)*f32::cos(self.tilt),
            self.radius*f32::sin(self.tilt),
            self.radius*f32::cos(self.camera_angle)*f32::cos(self.tilt),
            )
            +self.player_pos
    }
    pub fn proj_mat(&mut self) -> Mat4 {
        Mat4::perspective_rh(self.fov.into(), self.aspect.into(), self.near.into(), self.far.into())
    }
}
