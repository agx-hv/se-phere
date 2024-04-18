extern crate glam;
use glam::f32::{Mat4, Vec3A};
use glam::vec3a;
use std::f32::consts::PI;

pub struct CameraBare {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraBare {
    pub fn proj_mat(&mut self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.into(),
            self.aspect.into(),
            self.near.into(),
            self.far.into(),
        )
    }
}

pub struct FPSCamera {
    pub eye: Vec3A,
    pub up: Vec3A,
    pub target: Vec3A,
    pub camera_bare: CameraBare,
}

impl FPSCamera {
    pub fn new(eye: Vec3A, up: Vec3A, target: Vec3A, aspect: f32) -> Self {
        FPSCamera {
            eye,
            up,
            target,
            camera_bare: CameraBare {
                fov: PI / 3.0,
                aspect,
                near: 0.01,
                far: 100.0,
            },
        }
    }

    pub fn update(eye: Vec3A, up: Vec3A, target: Vec3A, aspect: f32, old_cam: FPSCamera) -> Self {
        FPSCamera {
            eye,
            up,
            target,
            camera_bare: CameraBare {
                fov: old_cam.camera_bare.fov,
                aspect,
                near: old_cam.camera_bare.near,
                far: old_cam.camera_bare.far,
            },
        }
    }

    pub fn view_mat(&mut self) -> Mat4 {
        Mat4::look_at_rh(self.eye.into(), self.target.into(), self.up.into())
    }

    pub fn proj_mat(&mut self) -> Mat4 {
        self.camera_bare.proj_mat()
    }
}

pub struct PlayerCamera {
    pub player_pos: Vec3A, //players coords
    pub camera_angle: f32, // 0 to 2pi, 0 is behind player
    pub tilt: f32,         // 0 to pi - tilt from ground to bird's eye
    pub radius: f32,       // camera distance away from player
    pub camera_bare: CameraBare,
}

impl PlayerCamera {
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

    pub fn view_mat(&mut self) -> Mat4 {
        if self.radius < 0.1 {
            self.radius = 0.1
        }
        if self.camera_angle < 0.0 {
            self.camera_angle += 2.0 * PI; // allows for camera to spin horinzontaly constantly around player while preventing int overflow
        }
        self.camera_angle = self.camera_angle % (2.0 * PI);
        if self.tilt > PI / 2.0 {
            self.tilt = PI / 2.0; // clip max tilt to 90deg
        } else if self.tilt < 1e-6 {
            self.tilt = 1e-6; // prevent edge case of completely flat camera
        }

        Mat4::look_at_rh(self.eye().into(), self.player_pos.into(), self.up().into())
    }

    pub fn eye(&self) -> Vec3A {
        vec3a(
            self.radius * f32::sin(self.camera_angle) * f32::cos(self.tilt),
            self.radius * f32::sin(self.tilt),
            self.radius * f32::cos(self.camera_angle) * f32::cos(self.tilt),
        ) + self.player_pos
    }

    pub fn up(&self) -> Vec3A {
        Vec3A::normalize(vec3a(
            self.radius * f32::sin(self.camera_angle) * -f32::sin(self.tilt),
            self.radius * f32::cos(self.tilt),
            self.radius * f32::cos(self.camera_angle) * -f32::sin(self.tilt),
        ))
    }

    pub fn proj_mat(&mut self) -> Mat4 {
        self.camera_bare.proj_mat()
    }
}
