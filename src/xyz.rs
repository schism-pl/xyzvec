use std::{
    fmt::{self, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

// TODO: parameterize by floating point length / fixed-point
// TODO: simd support?

#[derive(Clone, PartialEq, Copy)]
pub struct XYZArr {
    inner: [f64; 3],
}

impl XYZArr {
    pub fn new(inner: [f64; 3]) -> Self {
        Self { inner }
    }

    pub fn zeroes() -> Self {
        Self { inner: [0.0; 3] }
    }

    pub fn x(&self) -> f64 {
        self.inner[0]
    }

    pub fn y(&self) -> f64 {
        self.inner[1]
    }

    pub fn z(&self) -> f64 {
        self.inner[2]
    }

    pub fn scale_by(&self, d: f64) -> Self {
        let mut r = Self::zeroes();
        r.inner[0] = self.inner[0] * d;
        r.inner[1] = self.inner[1] * d;
        r.inner[2] = self.inner[2] * d;
        r
    }

    pub fn div_by(&self, d: f64) -> Self {
        let mut r = Self::zeroes();
        r.inner[0] = self.inner[0] / d;
        r.inner[1] = self.inner[1] / d;
        r.inner[2] = self.inner[2] / d;
        r
    }

    pub fn translated_by(&self, x: f64, y: f64, z: f64) -> Self {
        let new_x = self.x() + x;
        let new_y = self.y() + y;
        let new_z = self.z() + z;
        Self {
            inner: [new_x, new_y, new_z],
        }
    }

    pub fn l1_norm(&self) -> f64 {
        self.x() + self.y() + self.z()
    }

    pub fn l2_norm(&self) -> f64 {
        self.l2_norm_sqd().sqrt()
    }

    pub fn l2_norm_sqd(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn cross_prod(&self, other: Self) -> Self {
        let x: f64 = self.x() * other.y() - self.y() * other.x();
        let y: f64 = self.y() * other.z() - self.z() * other.y();
        let z: f64 = self.z() * other.x() - self.x() * other.z();
        Self::new([x, y, z])
    }

    pub fn dot_prod(&self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross_prod_magnitude_sqd(&self, other: Self) -> f64 {
        self.cross_prod(other).l2_norm_sqd()
    }

    // fn rotated_by_3d(&self, _other: Self, _theta: f64) -> Self {
    //     panic!("Reenable if we do 3d");
    //     // let c  = theta.cos();
    //     // let s = theta.sin();

    //     // let cross_prod = self.cross_prod(other);
    //     // let x = ((self.x() - other.x()*cross_prod))*(c - 1.0) + (other.z()*self.y() - other.y()*self.z())*s;
    //     // let y = ((self.y() - other.y()*cross_prod))*(c - 1.0) + (other.x()*self.z() - other.z()*self.x())*s;
    //     // let z = ((self.z() - other.z()*cross_prod))*(c - 1.0) + (other.y()*self.x() - other.x()*self.y())*s;
    //     // Self::new([x,y,z])
    // }
}

impl Add for XYZArr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        Self::new([x, y, z])
    }
}

impl AddAssign for XYZArr {
    fn add_assign(&mut self, other: Self) {
        self.inner[0] += other.x();
        self.inner[1] += other.y();
        self.inner[2] += other.z();
    }
}

impl Sub for XYZArr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        Self::new([x, y, z])
    }
}

impl SubAssign for XYZArr {
    fn sub_assign(&mut self, other: Self) {
        self.inner[0] -= other.x();
        self.inner[1] -= other.y();
        self.inner[2] -= other.z();
    }
}

impl fmt::Debug for XYZArr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x(), self.y(), self.z())
    }
}

impl fmt::Display for XYZArr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x(), self.y(), self.z())
    }
}
