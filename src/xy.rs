use std::{
    fmt::{self, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::VecInner;

#[derive(Clone, PartialEq, Copy)]
pub struct XYVec<T> {
    inner: [T; 2],
}

impl<T: VecInner> XYVec<T> {
    pub fn new(inner: [T; 2]) -> Self {
        Self { inner }
    }

    pub fn x(&self) -> T {
        self.inner[0]
    }

    pub fn y(&self) -> T {
        self.inner[1]
    }

    pub fn scale_by(&self, d: T) -> Self {
        let x = self.x() * d;
        let y = self.y() * d;
        Self::new([x, y])
    }

    pub fn div_by(&self, d: T) -> Self {
        let x = self.x() / d;
        let y = self.y() / d;
        Self::new([x, y])
    }

    pub fn l1_norm(&self) -> T {
        self.x() + self.y()
    }

    pub fn l2_norm_sqd(&self) -> T {
        self.x() * self.x() + self.y() * self.y()
    }

    pub fn cross_prod(&self, other: Self) -> T {
        self.x() * other.y() - self.y() * other.x()
    }

    pub fn dot_prod(&self, other: Self) -> T {
        self.x() * other.x() + self.y() * other.y()
    }

    pub fn translated_by(&self, x: T, y: T) -> Self {
        let new_x = self.x() + x;
        let new_y = self.y() + y;
        Self {
            inner: [new_x, new_y],
        }
    }
}

impl<T: VecInner> Add for XYVec<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        Self::new([x, y])
    }
}

impl<T: VecInner> AddAssign for XYVec<T> {
    fn add_assign(&mut self, other: Self) {
        self.inner[0] += other.x();
        self.inner[1] += other.y()
    }
}

impl<T: VecInner> Sub for XYVec<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        Self::new([x, y])
    }
}

impl<T: VecInner> SubAssign for XYVec<T> {
    fn sub_assign(&mut self, other: Self) {
        self.inner[0] -= other.x();
        self.inner[1] -= other.y()
    }
}

impl<T: VecInner> fmt::Debug for XYVec<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x(), self.y())
    }
}

impl<T: VecInner> fmt::Display for XYVec<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x(), self.y())
    }
}

impl XYVec<f32> {
    pub fn l2_norm(&self) -> f32 {
        self.l2_norm_sqd().sqrt()
    }

    pub fn zeroes() -> Self {
        Self { inner: [0.0; 2] }
    }

    pub fn rotated_by(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();

        let x = (self.x() * c - self.y() * s) - self.x();
        let y = self.x() * s + self.y() * c - self.y();
        Self::new([x, y])
    }
}

impl XYVec<f64> {
    pub fn l2_norm(&self) -> f64 {
        self.l2_norm_sqd().sqrt()
    }

    pub fn zeroes() -> Self {
        Self { inner: [0.0; 2] }
    }

    pub fn rotated_by(&self, theta: f64) -> Self {
        let c = theta.cos();
        let s = theta.sin();

        let x = (self.x() * c - self.y() * s) - self.x();
        let y = self.x() * s + self.y() * c - self.y();
        Self::new([x, y])
    }
}
