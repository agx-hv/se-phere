extern crate glm;
use glm::*;
use glm::ext::*;
use crate::utils::*;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vector3<f32>,
    pub center: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    pub vec: Vector3<f32>,
}

impl Camera {
    pub fn view_mat(&mut self) -> Matrix4<f32> {
        look_at::<f32>(self.eye, self.center, self.up)
    }
    pub fn proj_mat(&mut self) -> Matrix4<f32> {
        perspective::<f32>(self.fov, self.aspect, self.near, self.far)
    }

    pub fn mvhelper(&mut self, p_pos: Vector3<f32>, p_vec: Vector3<f32>) {
        xyz_plus_xyz(&mut self.vec,p_vec);

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
    pub player_pos: Vector3<f32>,
    pub camera_angle: f32, // 0 to 2pi
    pub tilt: f32, //0 to pi pls
    pub radius: f32,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    
}

impl PlayerCamera{
    pub fn view_mat(&mut self) -> Matrix4<f32> {
        if self.camera_angle<0.0{
            self.camera_angle += 2.0*PI;
        }
        self.camera_angle=self.camera_angle%(2.0*PI);
        if self.tilt > PI/2.0{
            self.tilt = PI/2.0;
        }
        else if self.tilt < 1e-6{
            self.tilt = 1e-6; //prevent edge case of completely flat camera
        }
        let eye: Vector3<f32> = glm::vec3(
            self.radius*sin(self.camera_angle)*cos(self.tilt),
            self.radius*sin(self.tilt),
            self.radius*cos(self.camera_angle)*cos(self.tilt),
            )
            +self.player_pos;

        let up: Vector3<f32> = glm::normalize(glm::vec3(
            self.radius*sin(self.camera_angle)*-sin(self.tilt),
            self.radius*sin(self.tilt),
            self.radius*cos(self.camera_angle)*-sin(self.tilt),
            )
            );

        look_at::<f32>(eye, self.player_pos, up)
        
    }
    pub fn proj_mat(&mut self) -> Matrix4<f32> {
        perspective::<f32>(self.fov, self.aspect, self.near, self.far)
    }
}
