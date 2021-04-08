use std::cmp::PartialEq;
use std::ops::{Add, Neg};

#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn null() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_different_vec3_are_not_equal() {
        assert_ne!(Vec3::new(1.0, 2.0, 3.0), Vec3::null());
    }

    #[test]
    fn null_vector_is_equal_to_null_vector() {
        assert_eq!(Vec3::null(), Vec3::null());
    }

    #[test]
    fn negation_works() {
        assert_eq!(-Vec3::new(-1.0, -2.0, 3.0), Vec3::new(1.0, 2.0, -3.0));
    }

    #[test]
    fn add_two_vec3() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 4.0, 6.0)
        );
    }
}
