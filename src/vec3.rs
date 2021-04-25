use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3-dimensional vector.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

/// Aliases of [`Vec3`] representing an 3-D point.
pub type Point3 = Vec3;

impl Vec3 {
    /// Constructs a new `Vec3`.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Construct the zero vector.
    ///
    /// Zero vector is a vector of length 0 and whose components are all 0.
    pub fn zero() -> Vec3 {
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

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the length of the vector using Euclidian norm.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the normalized vector.
    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }

    /// Returns the dot product of the vector with another.
    /// See [dot product on Wikipedia](https://en.wikipedia.org/wiki/Dot_product).
    pub fn dot(&self, &rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Returns the cross product of the vector with another.
    /// See [cross product on Wikipedia](https://en.wikipedia.org/wiki/Cross_product).
    pub fn cross(&self, &rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partial_eq_works_with_not_equals_vec3() {
        assert_ne!(Vec3::new(1.0, 2.0, 3.0), Vec3::zero());
    }

    #[test]
    fn partial_eq_works_with_equals_vec3() {
        assert_eq!(Vec3::zero(), Vec3::zero());
    }

    #[test]
    fn length_works() {
        assert_eq!(Vec3::new(4.0, 2.0, 4.0).length(), 6.0);
    }

    #[test]
    fn normalized_works() {
        assert_eq!(
            Vec3::new(4.0, 2.0, 4.0).normalized(),
            Vec3::new(4.0 / 6.0, 2.0 / 6.0, 4.0 / 6.0)
        );
    }

    #[test]
    fn dot_product_works() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 2.0).dot(&Vec3::new(1.0, 2.0, 3.0)),
            11.0
        );
    }

    #[test]
    fn cross_product_works() {
        assert_eq!(
            Vec3::new(1.0, 4.0, 3.0).cross(&Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(6.0, 0.0, -2.0)
        );
    }

    #[test]
    fn negation_works() {
        assert_eq!(-Vec3::new(-1.0, -2.0, 3.0), Vec3::new(1.0, 2.0, -3.0));
    }

    #[test]
    fn add_vec3_with_vec3_works() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 4.0, 6.0)
        );
    }

    #[test]
    fn add_assign_vec3_with_vec3_works() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0))
    }

    #[test]
    fn sub_vec3_with_vec3_works() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) - Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn sub_assign_vec3_with_vec3_works() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v -= Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn mul_by_vec3_works() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) * Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn mul_vec3_by_f64_works() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn mul_f64_by_vec3_works() {
        assert_eq!(2.0 * Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn mul_assign_vec3_by_f64_works() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0))
    }

    #[test]
    fn div_vec3_by_f64_works() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn div_assign_vec3_by_f64_works() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;
        assert_eq!(v, Vec3::new(1.0, 2.0, 3.0))
    }
}
