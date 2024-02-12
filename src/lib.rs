pub mod xy;
pub mod xyz;
// TODO: docs.rs
// TODO: tests with f64, f32, fixed point
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

pub use xy::XYVec;
pub use xyz::XYZVec;

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
            + Div<Output = Self>,
    > VecInner for V
{
}
