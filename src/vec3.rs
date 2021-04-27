use std::{
    cmp::PartialEq,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign},
};

use crate::random;

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

    /// Constructs the zero vector.
    ///
    /// Zero vector is a vector of length 0 and whose components are all 0.
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Constructs a random vector.
    pub fn random() -> Vec3 {
        Vec3 {
            x: random::random(),
            y: random::random(),
            z: random::random(),
        }
    }

    /// Constructs a random vector whose components are all in the given range.
    pub fn random_range(range: Range<f64>) -> Vec3 {
        Vec3 {
            x: random::random_range(range.clone()),
            y: random::random_range(range.clone()),
            z: random::random_range(range),
        }
    }

    /// Constructs a random vector inside a unit radius sphere.
    pub fn random_in_unit_sphere() -> Vec3 {
        let range = -1.0..1.0;

        let mut vector = Vec3::random_range(range.clone());
        while vector.length_squared() >= 1.0 {
            vector = Vec3::random_range(range.clone());
        }

        vector
    }

    /// Constructs a random vector inside a unit radius disk.
    pub fn random_in_unit_disk() -> Vec3 {
        let range = -1.0..1.0;

        let mut vector = Vec3::new(
            random::random_range(range.clone()),
            random::random_range(range.clone()),
            0.0,
        );
        while vector.length_squared() >= 1.0 {
            vector = Vec3::new(
                random::random_range(range.clone()),
                random::random_range(range.clone()),
                0.0,
            );
        }

        vector
    }

    /// Constructs a normalized random vector inside a unit radius sphere.
    ///
    /// This is a shortcut for `Vec3::random_in_unit_sphere().normalized()`.
    pub fn random_normalized() -> Vec3 {
        Vec3::random_in_unit_sphere().normalized()
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

    /// Returns true if the vector is close to zero in all dimensions.
    pub fn is_near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    /// Returns the normalized vector.
    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }

    /// Returns the reflected vector after intersecting with a surface, `n`
    /// being the surface normal.
    pub fn reflected(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * *n
    }

    /// Returns the refracted vector after intersecting with a surface, `n`
    /// being the surface normal. `eta` is the refractive index of the surface
    /// material.
    ///
    /// See [Sneel's law on Wikipedia](https://en.wikipedia.org/wiki/Snell%27s_law).
    pub fn refracted(&self, n: &Vec3, eta: f64) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let ray_out_perp = eta * (*self + cos_theta * *n);
        let ray_out_parallel = -(1.0 - ray_out_perp.length_squared()).abs().sqrt() * *n;
        ray_out_perp + ray_out_parallel
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
    fn is_near_zero_works_with_large_vec3() {
        assert!(!Vec3::new(8.0, 8.0, 8.0).is_near_zero());
    }

    #[test]
    fn is_near_zero_works_with_near_zero_vec3() {
        assert!(!Vec3::new(0.0, 1e-8, 8.0).is_near_zero());
    }

    #[test]
    fn normalized_works() {
        assert_eq!(
            Vec3::new(4.0, 2.0, 4.0).normalized(),
            Vec3::new(4.0 / 6.0, 2.0 / 6.0, 4.0 / 6.0)
        );
    }

    #[test]
    fn reflected_works() {
        assert_eq!(
            Vec3::new(2.0, -2.0, 0.0).reflected(&Vec3::new(0.0, 1.0, 0.0)),
            Vec3::new(2.0, 2.0, 0.0)
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
