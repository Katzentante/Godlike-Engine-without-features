use crate::maths::vec3::Vec3;

pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,

    pub eye: Vec3,
    pub at: Vec3,
    pub up: Vec3,
}


