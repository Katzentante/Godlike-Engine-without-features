use crate::maths::vec3::Vec3;

pub struct PerspectiveCamera {
    pub fovy: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,

    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

pub struct OrthographicCamera{

}

