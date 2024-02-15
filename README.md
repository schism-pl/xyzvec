# XYZVec
Statically allocated fixed-size vectors for working in 2 and 3 dimensions.
These vectors are parameterized over number representations, so the library \
works equally well for floating-point and fixed-point arithmetic.

`f32`, `f64`, and fixed-point (using the `fixed` crate) have been extensively tested. 
Other number representations may work, but have not been tested.

```Rust
use xyzvec::XYZVec;

fn main(){
    // define an initial position
    let pos = XYZVec::new([1.0, 1.0, 1.0]);
    // define a velocity factor 
    let v = XYZVec::new([2.0, 3.0, 4.0]);
    // calculate new position (x + vt) after 2 seconds
    let new_pos = pos + v.scale_by(2.0);
    // prints (5.0, 7.0, 9.0) 
    println!("{}", new_pos);
}

```