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

    /// `x` component of XYVec
    pub fn x(&self) -> T {
        self.inner[0]
    }

    /// `y` component of XYVec
    pub fn y(&self) -> T {
        self.inner[1]
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYVec::new([1.0f64, -0.5f64]);
    ///    let scaled_v = v.scale_by(5.0);
    ///    assert_relative_eq!(scaled_v.x(), 5.0);
    ///    assert_relative_eq!(scaled_v.y(), -2.5);
    /// ```
    pub fn scale_by(&self, d: T) -> Self {
        let x = self.x() * d;
        let y = self.y() * d;
        Self::new([x, y])
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYVec::new([1.0f64, -0.5f64]);
    ///    let scaled_v = v.div_by(0.2);
    ///    assert_relative_eq!(scaled_v.x(), 5.0);
    ///    assert_relative_eq!(scaled_v.y(), -2.5);
    /// ```
    pub fn div_by(&self, d: T) -> Self {
        let x = self.x() / d;
        let y = self.y() / d;
        Self::new([x, y])
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    //     let v = XYVec::new([1.0f64, -0.5f64]);
    //     assert_relative_eq!(v.l1_norm(), 0.5);
    /// ```
    pub fn l1_norm(&self) -> T {
        self.x() + self.y()
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    //     let v = XYVec::new([1.0f64, -0.5f64]);
    //     assert_relative_eq!(v.l2_norm_sqd(), 1.25);
    /// ```
    pub fn l2_norm_sqd(&self) -> T {
        self.x() * self.x() + self.y() * self.y()
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYVec::new([1.0f64, -0.5f64]);
    ///    let w = XYVec::new([-2.0f64, 0.0f64]);
    ///    assert_relative_eq!(v.cross_prod(w), -1.0);
    /// ```
    pub fn cross_prod(&self, other: Self) -> T {
        self.x() * other.y() - self.y() * other.x()
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYVec::new([1.0f64, -0.5f64]);
    ///    let w = XYVec::new([-2.0f64, 0.0f64]);
    ///    assert_relative_eq!(v.cross_prod_sqd(w), 1.0);
    /// ```
    pub fn cross_prod_sqd(&self, other: Self) -> T {
        self.cross_prod(other) * self.cross_prod(other)
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYVec::new([1.0f64, -0.5f64]);
    ///    let w = XYVec::new([-2.0f64, 0.0f64]);
    ///    assert_relative_eq!(v.dot_prod(w), -2.0);
    /// ```
    pub fn dot_prod(&self, other: Self) -> T {
        self.x() * other.x() + self.y() * other.y()
    }

    /// ```   
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;
    ///
    ///    let v = XYVec::new([1.0f64, -0.5f64]);
    ///    let scaled_v = v.translate_by(1.0, 1.0);
    ///    assert_relative_eq!(scaled_v.x(), 2.0);
    ///    assert_relative_eq!(scaled_v.y(), 0.5);
    /// ```
    pub fn translate_by(&self, x: T, y: T) -> Self {
        let new_x = self.x() + x;
        let new_y = self.y() + y;
        Self {
            inner: [new_x, new_y],
        }
    }

    ///```
    ///    use xyzvec::XYVec;
    ///    use approx::assert_relative_eq;    
    //
    //     let v = XYVec::new([1.0f64, -0.5f64]);
    //     assert_relative_eq!(v.sum(), 0.5);
    //
    // ```
    pub fn sum(&self) -> T {
        self.x() + self.y()
    }

    /// ```
    ///     use xyzvec::XYVec;
    ///     use approx::assert_relative_eq;    
    ///     
    ///     let v = XYVec::new([1.0f64, -0.5f64]);
    ///     let v2: XYVec<f64> = v.iter().map(|a| a + 1.0).collect();
    ///     assert_relative_eq!(v2.x(), 2.0);
    ///     assert_relative_eq!(v2.y(), 0.5);
    /// ```
    pub fn iter(&self) -> XYIterator<T> {
        XYIterator::new(self)
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

pub struct XYIterator<'a, T: VecInner> {
    vec: &'a XYVec<T>,
    index: u8,
}

impl<'a, T: VecInner> XYIterator<'a, T> {
    fn new(vec: &'a XYVec<T>) -> Self {
        Self { vec, index: 0 }
    }
}

impl<'a, T: VecInner> Iterator for XYIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some(self.vec.x()),
            2 => Some(self.vec.y()),
            _ => None,
        }
    }
}

/// Build XYVec from iterator of size two.
/// TODO: check for errors better
impl<T: VecInner> FromIterator<T> for XYVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        let x = i.next().unwrap();
        let y = i.next().unwrap();
        XYVec::new([x, y])
    }
}

#[cfg(test)]
mod tests {
    use crate::XYVec;
    use approx::assert_relative_eq;
    use fixed::types::I28F4;

    #[test]
    fn norms_f64() {
        let v = XYVec::new([1.0f64, -0.5f64]);
        assert_relative_eq!(v.l2_norm(), 1.25f64.sqrt());
    }

    #[test]
    fn scale_f32() {
        let v = XYVec::new([1.0f32, -0.5f32]);
        let scaled_v = v.scale_by(5.0);
        assert_relative_eq!(scaled_v.x(), 5.0);
        assert_relative_eq!(scaled_v.y(), -2.5);
    }

    #[test]
    fn div_f32() {
        let v = XYVec::new([1.0f32, -0.5f32]);
        let scaled_v = v.div_by(0.2);
        assert_relative_eq!(scaled_v.x(), 5.0);
        assert_relative_eq!(scaled_v.y(), -2.5);
    }

    #[test]
    fn translate_f32() {
        let v = XYVec::new([1.0f32, -0.5f32]);
        let scaled_v = v.translate_by(1.0, 1.0);
        assert_relative_eq!(scaled_v.x(), 2.0);
        assert_relative_eq!(scaled_v.y(), 0.5);
    }

    #[test]
    fn norms_f32() {
        let v = XYVec::new([1.0f32, -0.5f32]);
        assert_relative_eq!(v.l1_norm(), 0.5);
        assert_relative_eq!(v.l2_norm_sqd(), 1.25);
        assert_relative_eq!(v.l2_norm(), 1.25f32.sqrt());
    }

    #[test]
    fn cross_prod_f32() {
        let v = XYVec::new([1.0f32, -0.5f32]);
        let w = XYVec::new([-2.0f32, 0.0f32]);
        assert_relative_eq!(v.cross_prod(w), -1.0);
        assert_relative_eq!(v.cross_prod_sqd(w), 1.0);
    }

    #[test]
    fn dot_prod_f32() {
        let v = XYVec::new([1.0f32, -0.5f32]);
        let w = XYVec::new([-2.0f32, 0.0f32]);
        assert_relative_eq!(v.dot_prod(w), -2.0);
    }

    #[test]
    fn scale_fixed() {
        let v = XYVec::new([I28F4::from_num(1.0), I28F4::from_num(-0.5)]);
        let scaled_v = v.scale_by(I28F4::from_num(5.0));
        assert_eq!(scaled_v.x(), 5.0);
        assert_eq!(scaled_v.y(), -2.5);
    }

    #[test]
    fn div_fixed() {
        let v = XYVec::new([I28F4::from_num(1.0), I28F4::from_num(-0.5)]);
        let scaled_v = v.div_by(I28F4::from_num(0.25));
        assert_eq!(scaled_v.x(), 4.0);
        assert_eq!(scaled_v.y(), -2.0);
    }

    #[test]
    fn translate_fixed() {
        let v = XYVec::new([I28F4::from_num(1.0), I28F4::from_num(-0.5)]);
        let scaled_v = v.translate_by(I28F4::from_num(1.0), I28F4::from_num(1.0));
        assert_eq!(scaled_v.x(), 2.0);
        assert_eq!(scaled_v.y(), 0.5);
    }

    #[test]
    fn norms_fixed() {
        let v = XYVec::new([I28F4::from_num(1.0), I28F4::from_num(-0.5)]);
        assert_eq!(v.l1_norm(), 0.5);
        assert_eq!(v.l2_norm_sqd(), 1.25);
        //assert_relative_eq!(v.l2_norm(), 1.25f64.sqrt()); // not currently supported on fixed point
    }

    #[test]
    fn cross_prod_fixed() {
        let v = XYVec::new([I28F4::from_num(1.0), I28F4::from_num(-0.5)]);
        let w = XYVec::new([I28F4::from_num(-2.0), I28F4::from_num(-0.0)]);
        assert_eq!(v.cross_prod(w), -1.0);
        assert_eq!(v.cross_prod_sqd(w), 1.0);
    }

    #[test]
    fn dot_prod_fixed() {
        let v = XYVec::new([I28F4::from_num(1.0), I28F4::from_num(-0.5)]);
        let w = XYVec::new([I28F4::from_num(-2.0), I28F4::from_num(-0.0)]);
        assert_eq!(v.dot_prod(w), -2.0);
    }
}
