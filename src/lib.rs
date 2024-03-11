pub mod xy;
pub mod xyz;
// TODO: comments / doctest
// TODO: tests with f64, f32, fixed point
// TODO: checked operations
// TODO: fixed point support
// TODO: SIMD support
// TODO: approximate equality for fixed point?
// TODO: add relative_eq for tuples for simpler assertions

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

pub use xy::XYVec;
pub use xyz::XYZVec;

pub trait CordicPhantomTrait {}
impl<Frac> CordicPhantomTrait for fixed::FixedI8<Frac> {}
impl<Frac> CordicPhantomTrait for fixed::FixedI16<Frac> {}
impl<Frac> CordicPhantomTrait for fixed::FixedI32<Frac> {}
impl<Frac> CordicPhantomTrait for fixed::FixedI64<Frac> {}

pub trait VecInner:
    Clone
    + Copy
    + Sized
    + Debug
    + Display
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
}
impl<
        V: Clone
            + Copy
            + Sized
            + Debug
            + Display
            + Add<Output = Self>
            + AddAssign
            + Sub<Output = Self>
            + SubAssign
            + Mul<Output = Self>
            + Div<Output = Self>
            + Neg<Output = Self>,
    > VecInner for V
{
}
