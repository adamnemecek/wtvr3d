//! # Quaternion
//! Quaternion implementation with useful methods

use super::vector::Vector3;
use std::ops::{Mul,MulAssign};

#[derive(Clone)]
pub struct Quaternion{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Quaternion {
    /// Returns the identity quaternion
    ///
    /// # Examples
    ///
    /// ```
    /// let id = Quaternion::identity();
    /// ```
    pub fn identity() -> Quaternion {
        Quaternion { x : 0.0, y : 0.0, z : 0.0, w : 1.0 }
    }

    /// Returns a zero-filled Quaternion
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = Quaternion::zero();
    /// ```
    pub fn zero() -> Quaternion {
        Quaternion { x : 0.0, y : 0.0, z : 0.0, w : 0.0 }
    }

    /// Returns a quaternion from an axis and an angle (in radians)
    ///
    /// # Examples
    ///
    /// ```
    /// let quat = Quaternion::from_axis_angle(Vector3::identity(), math::PI/3.0)
    /// ```
    pub fn from_axis_angle(axis : Vector3, angle : f32) -> Quaternion {
        let sin_half = (angle/2.0).sin();
        let mut res = Quaternion {
            x : sin_half * axis.x,
            y : sin_half * axis.y,
            z : sin_half * axis.z,
            w : (angle/2.0).cos()
        };
        res *= 1.0/res.magnitude();
        res
    }

    /// Tests whether two Quaternions are equal.
    ///
    /// # Examples
    ///
    /// ```
    /// let quat1 = Quaternion::from_axis_angle(Vector3(1.0,1.0,1.0), PI);
    /// let quat2 = Quaternion::from_axis_angle(Vector3(1.0,1.0,1.0), PI);
    /// assert!(quat1.equals(&quat2));
    /// ```
    pub fn equals(&self, quat : &Quaternion) -> bool {
        self.x == quat.x && self.y == quat.y && self.w == quat.w && self.z == quat.z
    }

    /// Returns the magnitude (or vector length) of the quaternion.
    fn magnitude(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, f : f32) -> Quaternion {
        &self*f
    }
}

impl<'a> Mul<f32> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, f : f32) -> Quaternion {
        let mut res = self.clone();
        res.x *= f;
        res.y *= f;
        res.z *= f;
        res.w *= f;
        res
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, quat : Quaternion) -> Quaternion {
        self*&quat
    }
}

impl<'a> Mul<&'a Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, quat : &'a Quaternion) -> Quaternion {
        let mut res = quat.clone();
        res.x *= self;
        res.y *= self;
        res.z *= self;
        res.w *= self;
        res
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, f : f32) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
        self.w *= f;
    }
}

impl<'a> Mul<&'a Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, quat : &'a Quaternion ) -> Quaternion {
        Quaternion {
            x : self.x * quat.w + self.w * quat.x + self.y * quat.z - self.z * quat.y,
            y : self.y * quat.w + self.w * quat.y + self.z * quat.x - self.x * quat.z,
            z : self.z * quat.w + self.w * quat.z + self.x * quat.y - self.y * quat.x,
            w : self.w * quat.w - self.x * quat.x - self.y * quat.y - self.z * quat.z,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, quat : Quaternion ) -> Quaternion {
        &self * &quat
    }
}

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, quat : Quaternion ) {
        self.x = self.x * quat.w + self.w * quat.x + self.y * quat.z - self.z * quat.y;
        self.y = self.y * quat.w + self.w * quat.y + self.z * quat.x - self.x * quat.z;
        self.z = self.z * quat.w + self.w * quat.z + self.x * quat.y - self.y * quat.x;
        self.w = self.w * quat.w - self.x * quat.x - self.y * quat.y - self.z * quat.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::vector::Vector3;
    use super::super::PI;

    #[test]
    fn identity() {
        let id = Quaternion::identity();
        assert_eq!(id.x, 0.0);
        assert_eq!(id.y, 0.0);
        assert_eq!(id.z, 0.0);
        assert_eq!(id.w, 1.0);
    }

    #[test]
    fn zero() {
        let id = Quaternion::zero();
        assert_eq!(id.x, 0.0);
        assert_eq!(id.y, 0.0);
        assert_eq!(id.z, 0.0);
        assert_eq!(id.w, 0.0);
    }

    #[test]
    fn from_axis_angle() {
        let quat = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        println!("{}, {}, {}, {}",quat.x,quat.y,quat.z,quat.w);
        assert_eq!(quat.x,0.7071068);
        assert_eq!(quat.w,0.7071068);
        assert_eq!(quat.z,0.0);
        assert_eq!(quat.y,0.0);
    }
    #[test]
    fn equals() {
        let quat1 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        let mut quat2 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        assert!(quat1.equals(&quat2));
        quat2.z = 2.0;
        assert!(!quat2.equals(&quat1));
    }

    #[test]
    fn mul() {
        let quat1 = Quaternion::identity();
        let quat2 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        let quat3 = &quat1 * &quat2;
        assert!(quat2.equals(&quat3));
        let quat4 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        let quat5 = &quat2 * &quat4;
        assert!((1.0 - quat5.x).abs() < 0.0001);
        assert_eq!(quat5.y, 0.0);
        assert_eq!(quat5.z, 0.0);
        assert_eq!(quat5.w, 0.0);
    }
}