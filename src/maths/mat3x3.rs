use std::ops::Mul;

use super::vec3::Vec3;

pub fn get_rot_z(alpha: f32) -> Matrix3x3 {
    Matrix3x3::new(
        alpha.cos(),
        -(alpha.sin()),
        0.0,
        alpha.sin(),
        alpha.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub fn get_rot_x(alpha: f32) -> Matrix3x3 {
    Matrix3x3::new(
        1.0,
        0.0,
        0.0,
        0.0,
        alpha.cos(),
        -(alpha.sin()),
        0.0,
        alpha.sin(),
        alpha.cos(),
    )
}

pub fn get_rot_y(alpha: f32) -> Matrix3x3 {
    Matrix3x3::new(
        alpha.cos(),
        0.0,
        alpha.sin(),
        0.0,
        1.0,
        0.0,
        -(alpha.sin()),
        0.0,
        alpha.cos(),
    )
}

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

impl Mul for &Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            w11: self.w11 * rhs.w11 + self.w12 * rhs.w21 + self.w13 * rhs.w31,
            w12: self.w11 * rhs.w12 + self.w12 * rhs.w22 + self.w13 * rhs.w32,
            w13: self.w11 * rhs.w13 + self.w12 * rhs.w23 + self.w13 * rhs.w33,
            w21: self.w21 * rhs.w11 + self.w22 * rhs.w21 + self.w23 * rhs.w31,
            w22: self.w21 * rhs.w12 + self.w22 * rhs.w22 + self.w23 * rhs.w32,
            w23: self.w21 * rhs.w13 + self.w22 * rhs.w23 + self.w23 * rhs.w33,
            w31: self.w31 * rhs.w11 + self.w32 * rhs.w21 + self.w33 * rhs.w31,
            w32: self.w31 * rhs.w12 + self.w32 * rhs.w22 + self.w33 * rhs.w32,
            w33: self.w31 * rhs.w13 + self.w32 * rhs.w23 + self.w33 * rhs.w33,
        }
    }
}
