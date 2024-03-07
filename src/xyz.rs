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

    /// `x` component of XYZVec
    pub fn x(&self) -> T {
        self.inner[0]
    }

    /// `y` component of XYZVec
    pub fn y(&self) -> T {
        self.inner[1]
    }

    /// `z` component of XYZVec
    pub fn z(&self) -> T {
        self.inner[2]
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    let scaled_v = v.scale_by(5.0);
    ///    assert_relative_eq!(scaled_v.x(), 5.0);
    ///    assert_relative_eq!(scaled_v.y(), 10.0);
    ///    assert_relative_eq!(scaled_v.z(), -2.5);
    /// ```
    pub fn scale_by(&self, d: T) -> Self {
        let x = self.x() * d;
        let y = self.y() * d;
        let z = self.z() * d;
        Self::new([x, y, z])
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    let scaled_v = v.div_by(0.2);
    ///    assert_relative_eq!(scaled_v.x(), 5.0);
    ///    assert_relative_eq!(scaled_v.y(), 10.0);
    ///    assert_relative_eq!(scaled_v.z(), -2.5);
    /// ```
    pub fn div_by(&self, d: T) -> Self {
        let x = self.x() / d;
        let y = self.y() / d;
        let z = self.z() / d;
        Self::new([x, y, z])
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    let scaled_v = v.translate_by(1.0, 1.0, 1.0);
    ///    assert_relative_eq!(scaled_v.x(), 2.0);
    ///    assert_relative_eq!(scaled_v.y(), 3.0);
    ///    assert_relative_eq!(scaled_v.z(), 0.5);
    /// ```
    pub fn translate_by(&self, x: T, y: T, z: T) -> Self {
        let new_x = self.x() + x;
        let new_y = self.y() + y;
        let new_z = self.z() + z;
        Self {
            inner: [new_x, new_y, new_z],
        }
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    assert_relative_eq!(v.l1_norm(), 2.5);
    /// ```
    pub fn l1_norm(&self) -> T {
        self.x() + self.y() + self.z()
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    assert_relative_eq!(v.l2_norm_sqd(), 5.25);
    /// ```
    pub fn l2_norm_sqd(&self) -> T {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    let w = XYZVec::new([-2.0f64, 0.5f64, 0.0f64]);
    ///    let cross_prod = v.cross_prod(w);
    ///    assert_relative_eq!(cross_prod.x(), 4.5);
    ///    assert_relative_eq!(cross_prod.y(), 0.25);
    ///    assert_relative_eq!(cross_prod.z(), 1.0);
    /// ```
    pub fn cross_prod(&self, other: Self) -> Self {
        let x: T = self.x() * other.y() - self.y() * other.x();
        let y: T = self.y() * other.z() - self.z() * other.y();
        let z: T = self.z() * other.x() - self.x() * other.z();
        Self::new([x, y, z])
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    let w = XYZVec::new([-2.0f64, 0.5f64, 0.0f64]);
    ///    assert_relative_eq!(v.dot_prod(w), -1.0);
    /// ```
    pub fn dot_prod(&self, other: Self) -> T {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// ```   
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///    let w = XYZVec::new([-2.0f64, 0.5f64, 0.0f64]);
    ///    let mag_sqd = v.cross_prod_magnitude_sqd(w);
    ///    assert_relative_eq!(mag_sqd, 21.3125);
    /// ```
    pub fn cross_prod_magnitude_sqd(&self, other: Self) -> T {
        self.cross_prod(other).l2_norm_sqd()
    }

    ///```
    ///    use xyzvec::XYZVec;
    ///    use approx::assert_relative_eq;    
    //
    //     let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    //     assert_relative_eq!(v.sum(), 2.5);
    //
    // ```
    pub fn sum(&self) -> T {
        self.x() + self.y() + self.z()
    }

    /// ```
    ///     use xyzvec::XYZVec;
    ///     use approx::assert_relative_eq;    
    ///     
    ///     let v = XYZVec::new([1.0f64, 2.0f64, -0.5f64]);
    ///     let v2: XYZVec<f64> = v.iter().map(|a| a + 1.0).collect();
    ///     assert_relative_eq!(v2.x(), 2.0);
    ///     assert_relative_eq!(v2.y(), 3.0);
    ///     assert_relative_eq!(v2.z(), 0.5);
    /// ```
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.inner.iter()
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

/// Build XYVec from iterator of size two.
/// TODO: check for errors better
impl<T: VecInner> FromIterator<T> for XYZVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        let x = i.next().unwrap();
        let y = i.next().unwrap();
        let z = i.next().unwrap();
        XYZVec::new([x, y, z])
    }
}

#[cfg(test)]
mod tests {
    use crate::XYZVec;
    use approx::assert_relative_eq;
    use fixed::types::I28F4;

    #[test]
    fn scale_f32() {
        let v = XYZVec::new([1.0f32, 2.0f32, -0.5f32]);
        let scaled_v = v.scale_by(5.0);
        assert_relative_eq!(scaled_v.x(), 5.0);
        assert_relative_eq!(scaled_v.y(), 10.0);
        assert_relative_eq!(scaled_v.z(), -2.5);
    }

    #[test]
    fn div_f32() {
        let v = XYZVec::new([1.0f32, 2.0f32, -0.5f32]);
        let scaled_v = v.div_by(0.2);
        assert_relative_eq!(scaled_v.x(), 5.0);
        assert_relative_eq!(scaled_v.y(), 10.0);
        assert_relative_eq!(scaled_v.z(), -2.5);
    }

    #[test]
    fn translate_f32() {
        let v = XYZVec::new([1.0f32, 2.0f32, -0.5f32]);
        let scaled_v = v.translate_by(1.0, 1.0, 1.0);
        assert_relative_eq!(scaled_v.x(), 2.0);
        assert_relative_eq!(scaled_v.y(), 3.0);
        assert_relative_eq!(scaled_v.z(), 0.5);
    }

    #[test]
    fn norms_f32() {
        let v = XYZVec::new([1.0f32, 2.0f32, -0.5f32]);
        assert_relative_eq!(v.l1_norm(), 2.5);
        assert_relative_eq!(v.l2_norm_sqd(), 5.25);
        assert_relative_eq!(v.l2_norm(), 5.25f32.sqrt());
    }

    #[test]
    fn cross_prod_f32() {
        let v = XYZVec::new([1.0f32, 2.0f32, -0.5f32]);
        let w = XYZVec::new([-2.0f32, 0.5f32, 0.0f32]);
        let cross_prod = v.cross_prod(w);
        assert_relative_eq!(cross_prod.x(), 4.5);
        assert_relative_eq!(cross_prod.y(), 0.25);
        assert_relative_eq!(cross_prod.z(), 1.0);
        let mag_sqd = v.cross_prod_magnitude_sqd(w);
        assert_relative_eq!(mag_sqd, 21.3125);
    }

    #[test]
    fn dot_prod_f32() {
        let v = XYZVec::new([1.0f32, 2.0f32, -0.5f32]);
        let w = XYZVec::new([-2.0f32, 0.5f32, 0.0f32]);
        assert_relative_eq!(v.dot_prod(w), -1.0);
    }

    #[test]
    fn scale_fixed() {
        let v = XYZVec::new([
            I28F4::from_num(1.0),
            I28F4::from_num(2.0),
            I28F4::from_num(-0.5),
        ]);
        let scaled_v = v.scale_by(I28F4::from_num(5.0));
        assert_eq!(scaled_v.x(), 5.0);
        assert_eq!(scaled_v.y(), 10.0);
        assert_eq!(scaled_v.z(), -2.5);
    }

    #[test]
    fn div_fixed() {
        let v = XYZVec::new([
            I28F4::from_num(1.0),
            I28F4::from_num(2.0),
            I28F4::from_num(-0.5),
        ]);
        let scaled_v = v.div_by(I28F4::from_num(0.25));
        assert_eq!(scaled_v.x(), 4.0);
        assert_eq!(scaled_v.y(), 8.0);
        assert_eq!(scaled_v.z(), -2.0);
    }

    #[test]
    fn translate_fixed() {
        let v = XYZVec::new([
            I28F4::from_num(1.0),
            I28F4::from_num(2.0),
            I28F4::from_num(-0.5),
        ]);
        let scaled_v = v.translate_by(
            I28F4::from_num(1.0),
            I28F4::from_num(1.0),
            I28F4::from_num(1.0),
        );
        assert_eq!(scaled_v.x(), 2.0);
        assert_eq!(scaled_v.y(), 3.0);
        assert_eq!(scaled_v.z(), 0.5);
    }

    #[test]
    fn norms_fixed() {
        let v = XYZVec::new([
            I28F4::from_num(1.0),
            I28F4::from_num(2.0),
            I28F4::from_num(-0.5),
        ]);
        assert_eq!(v.l1_norm(), 2.5);
        assert_eq!(v.l2_norm_sqd(), 5.25);
        //assert_relative_eq!(v.l2_norm(), 1.25f64.sqrt()); // not currently supported on fixed point
    }

    #[test]
    fn cross_prod_fixed() {
        let v = XYZVec::new([
            I28F4::from_num(1.0),
            I28F4::from_num(2.0),
            I28F4::from_num(-0.5),
        ]);
        let w = XYZVec::new([
            I28F4::from_num(-2.0),
            I28F4::from_num(0.5),
            I28F4::from_num(-0.0),
        ]);
        let cross_prod = v.cross_prod(w);
        assert_eq!(cross_prod.x(), 4.5);
        assert_eq!(cross_prod.y(), 0.25);
        assert_eq!(cross_prod.z(), 1.0);
        let mag_sqd = v.cross_prod_magnitude_sqd(w);
        assert_eq!(mag_sqd, 21.3125);
    }

    #[test]
    fn dot_prod_fixed() {
        let v = XYZVec::new([
            I28F4::from_num(1.0),
            I28F4::from_num(2.0),
            I28F4::from_num(-0.5),
        ]);
        let w = XYZVec::new([
            I28F4::from_num(-2.0),
            I28F4::from_num(0.5),
            I28F4::from_num(-0.0),
        ]);
        assert_eq!(v.dot_prod(w), -1.0);
    }
}
