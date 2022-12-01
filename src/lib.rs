pub mod base;
pub mod canvas;
pub mod scene;
pub mod util;

pub use cgmath;


// todo temp cannot figure out how to get `Quaternion::from(euler)` to compile
//      "the trait `Angle` is not implemented for `f64`"
use crate::cgmath::{Euler, Quaternion};
pub fn quaternion_from_euler(src: Euler<f64>) -> Quaternion<f64> {
    let s_x = src.x.sin() * 0.5;
    let c_x = src.x.cos() * 0.5;
    let s_y = src.y.sin() * 0.5;
    let c_y = src.y.cos() * 0.5;
    let s_z = src.z.sin() * 0.5;
    let c_z = src.z.cos() * 0.5;
    Quaternion::<f64>::new(
        -s_x * s_y * s_z + c_x * c_y * c_z,
        s_x * c_y * c_z + s_y * s_z * c_x,
        -s_x * s_z * c_y + s_y * c_x * c_z,
        s_x * s_y * c_z + s_z * c_x * c_y) // rem, still need to normalize this
}
