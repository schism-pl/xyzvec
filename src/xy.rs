use std::{
    fmt::{self, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, PartialEq, Copy)]
pub struct XYArr {
    inner: [f64; 2],
}

impl XYArr {
    pub fn new(inner: [f64; 2]) -> Self {
        Self { inner }
    }

    pub fn zeroes() -> Self {
        XYArr { inner: [0.0; 2] }
    }

    pub fn x(&self) -> f64 {
        self.inner[0]
    }

    pub fn y(&self) -> f64 {
        self.inner[1]
    }

    pub fn scale_by(&self, d: f64) -> Self {
        let mut r = Self::zeroes();
        r.inner[0] = self.inner[0] * d;
        r.inner[1] = self.inner[1] * d;
        r
    }

    pub fn div_by(&self, d: f64) -> Self {
        let mut r = Self::zeroes();
        r.inner[0] = self.inner[0] / d;
        r.inner[1] = self.inner[1] / d;
        r
    }

    // // pick XYArr from normal distribution
    // pub fn random(rng: &mut SmallRng) -> Self {
    //     let normal = Normal::new(0.0, 1.0).unwrap();
    //     let mut r = XYArr::zeroes();
    //     for idx in 0..2 {
    //         r.inner[idx] = normal.sample(rng);
    //     }
    //     r
    // }

    // // normalized radom XYArr
    // pub fn rand_unit_vector(rng: &mut SmallRng) -> Self {
    //     let rand_pos = XYArr::random(rng);
    //     let norm = rand_pos.norm();
    //     rand_pos.scalar_div(norm)
    // }

    pub fn l1_norm(&self) -> f64 {
        self.x() + self.y()
    }

    pub fn l2_norm(&self) -> f64 {
        self.l2_norm_sqd().sqrt()
    }

    pub fn l2_norm_sqd(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y()
    }

    pub fn cross_prod(&self, other: XYArr) -> f64 {
        self.x() * other.y() - self.y() * other.x()
    }

    pub fn dot_prod(&self, other: XYArr) -> f64 {
        self.x() * other.x() + self.y() * other.y()
    }

    pub fn translated_by(&self, x: f64, y: f64) -> Self {
        let new_x = self.x() + x;
        let new_y = self.y() + y;
        Self {
            inner: [new_x, new_y],
        }
    }

    // TODO: what are the units of theta? (update docs if its not radians)
    pub fn rotated_by(&self, theta: f64) -> Self {
        let c = theta.cos();
        let s = theta.sin();

        let x = (self.x() * c - self.y() * s) - self.x();
        let y = self.x() * s + self.y() * c - self.y();
        Self::new([x, y])
    }

    // // TODO: should probably generalize this
    // pub fn div_u32(self, other: [u32; 2]) -> Self {
    //     let mut r = XYArr::zeroes();
    //     for idx in 0..2 {
    //         r.inner[idx] = self.inner[idx] / other[idx] as f64;
    //     }
    //     r
    // }
}

impl Add for XYArr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        Self::new([x, y])
    }
}

impl AddAssign for XYArr {
    fn add_assign(&mut self, other: XYArr) {
        self.inner[0] += other.x();
        self.inner[1] += other.y()
    }
}

impl Sub for XYArr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        Self::new([x, y])
    }
}

impl SubAssign for XYArr {
    fn sub_assign(&mut self, other: XYArr) {
        self.inner[0] -= other.x();
        self.inner[1] -= other.y()
    }
}

impl fmt::Debug for XYArr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x(), self.y())
    }
}

impl fmt::Display for XYArr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x(), self.y())
    }
}
