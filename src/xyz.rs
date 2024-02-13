use std::{
    fmt::{self, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::VecInner;

#[derive(Clone, PartialEq, Copy)]
pub struct XYZVec<T> {
    inner: [T; 3],
}

impl<T: VecInner> XYZVec<T> {
    pub fn new(inner: [T; 3]) -> Self {
        Self { inner }
    }

    pub fn x(&self) -> T {
        self.inner[0]
    }

    pub fn y(&self) -> T {
        self.inner[1]
    }

    pub fn z(&self) -> T {
        self.inner[2]
    }

    pub fn scale_by(&self, d: T) -> Self {
        let x = self.x() * d;
        let y = self.y() * d;
        let z = self.z() * d;
        Self::new([x, y, z])
    }

    pub fn div_by(&self, d: T) -> Self {
        let x = self.x() / d;
        let y = self.y() / d;
        let z = self.z() / d;
        Self::new([x, y, z])
    }
    pub fn translated_by(&self, x: T, y: T, z: T) -> Self {
        let new_x = self.x() + x;
        let new_y = self.y() + y;
        let new_z = self.z() + z;
        Self {
            inner: [new_x, new_y, new_z],
        }
    }

    pub fn l1_norm(&self) -> T {
        self.x() + self.y() + self.z()
    }

    pub fn l2_norm_sqd(&self) -> T {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    // -----------------------------

    pub fn cross_prod(&self, other: Self) -> Self {
        let x: T = self.x() * other.y() - self.y() * other.x();
        let y: T = self.y() * other.z() - self.z() * other.y();
        let z: T = self.z() * other.x() - self.x() * other.z();
        Self::new([x, y, z])
    }

    pub fn dot_prod(&self, other: Self) -> T {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross_prod_magnitude_sqd(&self, other: Self) -> T {
        self.cross_prod(other).l2_norm_sqd()
    }
}

impl<T: VecInner> Add for XYZVec<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        Self::new([x, y, z])
    }
}

impl<T: VecInner> AddAssign for XYZVec<T> {
    fn add_assign(&mut self, other: Self) {
        self.inner[0] += other.x();
        self.inner[1] += other.y();
        self.inner[2] += other.z();
    }
}

impl<T: VecInner> Sub for XYZVec<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        Self::new([x, y, z])
    }
}

impl<T: VecInner> SubAssign for XYZVec<T> {
    fn sub_assign(&mut self, other: Self) {
        self.inner[0] -= other.x();
        self.inner[1] -= other.y();
        self.inner[2] -= other.z();
    }
}

impl<T: VecInner> fmt::Debug for XYZVec<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x(), self.y(), self.z())
    }
}

impl<T: VecInner> fmt::Display for XYZVec<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x(), self.y(), self.z())
    }
}

impl XYZVec<f32> {
    pub fn l2_norm(&self) -> f32 {
        self.l2_norm_sqd().sqrt()
    }

    pub fn zeroes() -> Self {
        Self { inner: [0.0; 3] }
    }
}

impl XYZVec<f64> {
    pub fn l2_norm(&self) -> f64 {
        self.l2_norm_sqd().sqrt()
    }

    pub fn zeroes() -> Self {
        Self { inner: [0.0; 3] }
    }
}

// fn rotated_by_3d(&self, _other: Self, _theta: T) -> Self {
//     panic!("Reenable if we do 3d");
//     // let c  = theta.cos();
//     // let s = theta.sin();

//     // let cross_prod = self.cross_prod(other);
//     // let x = ((self.x() - other.x()*cross_prod))*(c - 1.0) + (other.z()*self.y() - other.y()*self.z())*s;
//     // let y = ((self.y() - other.y()*cross_prod))*(c - 1.0) + (other.x()*self.z() - other.z()*self.x())*s;
//     // let z = ((self.z() - other.z()*cross_prod))*(c - 1.0) + (other.y()*self.x() - other.x()*self.y())*s;
//     // Self::new([x,y,z])
// }




#[cfg(test)]
mod tests {
    use crate::XYZVec;
    use approx::assert_relative_eq;

    #[test]
    fn scale_f64() {
        let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
        let scaled_v = v.scale_by(5.0);
        assert_relative_eq!(scaled_v.x(), 5.0);
        assert_relative_eq!(scaled_v.y(), 10.0);
        assert_relative_eq!(scaled_v.z(), -2.5);
    }

    #[test]
    fn div_f64() {
        let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
        let scaled_v = v.div_by(0.2);
        assert_relative_eq!(scaled_v.x(), 5.0);
        assert_relative_eq!(scaled_v.y(), 10.0);
        assert_relative_eq!(scaled_v.z(), -2.5);
    }

    #[test]
    fn translate_f64() {
        let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
        let scaled_v = v.translated_by(1.0, 1.0, 1.0);
        assert_relative_eq!(scaled_v.x(), 2.0);
        assert_relative_eq!(scaled_v.y(), 3.0);
        assert_relative_eq!(scaled_v.z(), 0.5);
    }

    #[test]
    fn norms_f64() {
        let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
        assert_relative_eq!(v.l1_norm(), 2.5);
        assert_relative_eq!(v.l2_norm_sqd(), 5.25);
        assert_relative_eq!(v.l2_norm(), 5.25f64.sqrt());
    }


}