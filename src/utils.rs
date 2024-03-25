
use glm::*;


pub fn xyz_plus_xyz(target: &mut Vector3<f32>, hunter: Vector3<f32>) {
    target.x += hunter.x;
    target.y += hunter.y;
    target.z += hunter.z;
}

pub fn xyz_times_n(target: &mut Vector3<f32>, n: f32) {
    target.x *= n;
    target.y *= n;
    target.z *= n;
}