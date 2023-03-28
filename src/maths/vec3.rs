use std::ops::{Mul, Add, Sub, Neg};

pub const IDENTITY_X: Vec3 = Vec3 {x: 1.0, y: 0.0, z: 0.0};
pub const IDENTITY_Y: Vec3 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
pub const IDENTITY_Z: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};
pub const ZERO: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z}
    }

    pub fn from_points(one: &Vec3, two: &Vec3) -> Self {
        Self {
            x: two.x - one.x,
            y: two.y - one.y,
            z: two.z - one.z
        }
    }
    pub fn dot_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> Option<f32> {
        let s = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if s == 0.0 {
            None
        } else {
            Some(s)
        }
    }

    pub fn cross_angle(&self, other: &Self) -> f32 {
        (self.dot_product(other) / (self.len().unwrap_or(0.0) * other.len().unwrap_or(0.0))).acos()
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl PartialEq for &Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

#[cfg(test)]
mod tests {
    // Import the necessary modules
    use super::Vec3;

    #[test]
    fn test_cross_angle() {
        #[rustfmt::ignore]
        let vec1 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
        let vec2 = Vec3 {x: 0.0, y: 0.0, z: 1.0};
        assert_eq!(vec1.cross_angle(&vec2), std::f32::consts::FRAC_PI_2);
    }

    #[test]
    fn test_length() {
        let vec1 = Vec3 {x: 3.0, y: 3.0, z: 3.0};
        assert_eq!(vec1.len(), Some((27.0f32).sqrt()));
    }

}
