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

impl PerspectiveCamera {
    // Z achsis of up points upward
    pub fn calc_up(&mut self) {
        if self.target.z - self.pos.z != 0.0 {
            self.up.x = self.target.x - self.pos.x;
            self.up.y = self.target.y - self.pos.y;
            self.up.z = -(self.up.x.powi(2) + self.up.y.powi(2)) / (self.target.z - self.pos.z);
        } else if self.target.y - self.pos.y != 0.0 { // ea.z is 0
            self.up.x = self.target.x - self.pos.x;
            self.up.z = self.target.z - self.pos.z;
            self.up.y = -(self.up.x.powi(2) + self.up.z.powi(2)) / (self.target.y - self.pos.y);
        } else { // ea.z and ea.y is 0
            self.up.z = self.target.z - self.pos.z;
            self.up.y = self.target.y - self.pos.y;
            self.up.x = -(self.up.z.powi(2) + self.up.y.powi(2)) / (self.target.x - self.pos.x);
        }
    }
}

pub struct OrthographicCamera {}
