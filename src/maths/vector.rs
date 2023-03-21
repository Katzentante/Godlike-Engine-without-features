use std::ops::Mul;

use sdl2::rect::Point;

pub const IDENTITY_X3: Vec3 = Vec3 {x: 1.0, y: 0.0, z: 0.0};
pub const IDENTITY_Y3: Vec3 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
pub const IDENTITY_Z3: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn from_points(one: &Vec3, two: &Vec3) -> Vec3 {
        Vec3 {
            x: two.x - one.x,
            y: two.y - one.y,
            z: two.z - one.z
        }
    }
    pub fn dot_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn cross_angle(&self, other: &Self) -> f32 {
        (self.dot_product(other) / (self.len() * other.len())).acos()
    }
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn dot_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y 
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn cross_angle(&self, other: &Self) -> f32 {
        (self.dot_product(other) / (self.len() * other.len())).acos()
    }
}

impl From<Vec2> for Point {
    fn from(value: Vec2) -> Self {
        Point::new(value.x as i32, value.y as i32)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
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
        assert_eq!(vec1.len(), (27.0f32).sqrt());
    }

}
