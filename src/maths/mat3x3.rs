use std::ops::Mul;

use super::vec3::Vec3;

pub struct Matrix3x3 {
    w11: f32,
    w12: f32,
    w13: f32,
    w21: f32,
    w22: f32,
    w23: f32,
    w31: f32,
    w32: f32,
    w33: f32,
}

impl Matrix3x3 {
    pub fn new(
        w11: f32,
        w12: f32,
        w13: f32,
        w21: f32,
        w22: f32,
        w23: f32,
        w31: f32,
        w32: f32,
        w33: f32,
    ) -> Self {
        Self {
            w11,
            w12,
            w13,
            w21,
            w22,
            w23,
            w31,
            w32,
            w33,
        }
    }
}

impl Mul<&Vec3> for &Matrix3x3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.w11 * rhs.x + self.w12 * rhs.y + self.w13 * rhs.z,
            y: self.w21 * rhs.x + self.w22 * rhs.y + self.w23 * rhs.z,
            z: self.w31 * rhs.x + self.w32 * rhs.y + self.w33 * rhs.z,
        }
    }
}
