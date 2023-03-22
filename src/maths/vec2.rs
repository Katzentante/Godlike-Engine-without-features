use std::ops::Mul;
use sdl2::rect::Point;

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

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
