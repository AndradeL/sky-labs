// Copyright (c) 2025 Lucas B. Andrade
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::f64;

use sky_labs::math::Vector3;

macro_rules! test_vector3_new {
    ($type:ty) => {
        let v = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        assert_eq!(v.x, 1 as $type);
        assert_eq!(v.y, 2 as $type);
        assert_eq!(v.z, 3 as $type);
    };
}

macro_rules! test_vector3_zero {
    ($type:ty) => {
        let v = Vector3::<$type>::zero();
        assert_eq!(v.x, 0 as $type);
        assert_eq!(v.y, 0 as $type);
        assert_eq!(v.z, 0 as $type);
    };
}

macro_rules! test_vector3_default {
    ($type:ty) => {
        let v: Vector3<$type> = Default::default();
        assert_eq!(v.x, 0 as $type);
        assert_eq!(v.y, 0 as $type);
        assert_eq!(v.z, 0 as $type);
    };
}

macro_rules! test_vector3_one {
    ($type:ty) => {
        let v = Vector3::<$type>::one();
        assert_eq!(v.x, 1 as $type);
        assert_eq!(v.y, 1 as $type);
        assert_eq!(v.z, 1 as $type);
    };
}

macro_rules! test_vector3_add {
    ($type:ty) => {
        let v1 = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        let v2 = Vector3::<$type>::new(4 as $type, 5 as $type, 6 as $type);
        let expected = Vector3::<$type>::new(5 as $type, 7 as $type, 9 as $type);
        let result = v1 + v2;
        assert_eq!(result, expected);
    };
}

macro_rules! test_vector3_sub {
    ($type:ty) => {
        let v1 = Vector3::<$type>::new(5 as $type, 7 as $type, 9 as $type);
        let v2 = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        let expected = Vector3::<$type>::new(4 as $type, 5 as $type, 6 as $type);
        let result = v1 - v2;
        assert_eq!(result, expected);
    };
}

macro_rules! test_vector3_dot {
    ($type:ty) => {
        let v1 = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        let v2 = Vector3::<$type>::new(4 as $type, 5 as $type, 6 as $type);
        let expected = 32 as $type; // 1*4 + 2*5 + 3*6
        let result = v1.dot(&v2);
        assert_eq!(result, expected);
    };
}

macro_rules! test_vector3_magnitude {
    ($type:ty) => {
        let v = Vector3::<$type>::new(3 as $type, 4 as $type, 0 as $type);
        let expected = 5.0; // sqrt(3^2 + 4^2 + 0^2)
        let result = v.magnitude();
        assert!((result - expected).abs() < 1e-6);
    };
}

macro_rules! test_vector3_zero_magnitude {
    ($type:ty) => {
        let v = Vector3::<$type>::new(0 as $type, 0 as $type, 0 as $type);
        assert_eq!(v.magnitude(), 0.0);
    };
}

macro_rules! test_vector3_scalar_multiplication {
    ($type:ty) => {
        let v = Vector3::<$type>::new(1 as $type, -2 as $type, 3 as $type);
        let result = v * 2 as $type;
        let expected = Vector3::<$type>::new(2 as $type, -4 as $type, 6 as $type);
        assert_eq!(result, expected);
        let result_div = v / 2 as $type;
        let expected_div = Vector3::<$type>::new(0.5 as $type, -1.0 as $type, 1.5 as $type);
        assert_eq!(result_div, expected_div);
    };
}

macro_rules! test_vector3_distance {
    ($type:ty) => {
        let v1 = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        let v2 = Vector3::<$type>::new(4 as $type, 6 as $type, 3 as $type);
        let expected = 5.0; // sqrt((4-1)^2 + (6-2)^2 + (3-3)^2)
        let result = v1.distance_to(&v2);
        assert!((result - expected).abs() < 1e-6);
    };
}

macro_rules! test_vector3_cross {
    ($type:ty) => {
        let v1 = Vector3::<$type>::new(1 as $type, 0 as $type, 0 as $type);
        let v2 = Vector3::<$type>::new(0 as $type, 1 as $type, 0 as $type);
        let expected = Vector3::<$type>::new(0 as $type, 0 as $type, 1 as $type);
        let result = v1.cross(&v2);
        assert_eq!(result, expected);
    };
}

macro_rules! test_vector3_rotate {
    ($type:ty, $rot:ident, $rad:expr, $expected:expr) => {
        let v = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        let expected = $expected;
        let angle = $rad;
        let result = v.$rot(angle);
        assert!((result.x - expected.x).abs() < 1e-6);
        assert!((result.y - expected.y).abs() < 1e-6);
        assert!((result.z - expected.z).abs() < 1e-6);
    };
}

macro_rules! test_vector3_normalize {
    ($type:ty) => {
        let v = Vector3::<$type>::new(3 as $type, 4 as $type, 0 as $type);
        assert!(!v.is_normalized());
        let expected = Vector3::<$type>::new(0.6 as $type, 0.8 as $type, 0.0 as $type); // normalized vector
        let result = v.normalize();
        assert!(result.is_normalized());
        assert!((result.x - expected.x).abs() < 1e-6);
        assert!((result.y - expected.y).abs() < 1e-6);
        assert!((result.z - expected.z).abs() < 1e-6);
    };
}

macro_rules! test_vector3_as_slice {
    ($type:ty) => {
        let v = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        let slice = v.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(std::mem::size_of_val(slice), 3 * std::mem::size_of::<$type>());
        assert_eq!(slice[0], 1 as $type);
        assert_eq!(slice[1], 2 as $type);
        assert_eq!(slice[2], 3 as $type);
    };
}

macro_rules! test_vector3_as_mut_slice {
    ($type:ty) => {
        let mut v = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        {
            let slice = v.as_mut_slice();
            assert_eq!(slice.len(), 3);
            assert_eq!(std::mem::size_of_val(slice), 3 * std::mem::size_of::<$type>());
            slice[0] = 10 as $type;
            slice[1] = 20 as $type;
            slice[2] = 30 as $type;
        }
        assert_eq!(v.x, 10 as $type);
        assert_eq!(v.y, 20 as $type);
        assert_eq!(v.z, 30 as $type);
    };
}

macro_rules! test_vector3_as_ptr {
    ($type:ty) => {
        let v = Vector3::<$type>::new(4 as $type, 5 as $type, 6 as $type);
        unsafe {
            let ptr = v.as_ptr();
            assert_eq!(*ptr.offset(0), 4 as $type);
            assert_eq!(*ptr.offset(1), 5 as $type);
            assert_eq!(*ptr.offset(2), 6 as $type);
        }
    };
}

macro_rules! test_vector3_as_mut_ptr {
    ($type:ty) => {
        let mut v = Vector3::<$type>::new(7 as $type, 8 as $type, 9 as $type);
        unsafe {
            let ptr = v.as_mut_ptr();
            *ptr.offset(0) = 70 as $type;
            *ptr.offset(1) = 80 as $type;
            *ptr.offset(2) = 90 as $type;
        }
        assert_eq!(v.x, 70 as $type);
        assert_eq!(v.y, 80 as $type);
        assert_eq!(v.z, 90 as $type);
    };
}

#[test]
fn test_vector3_new() {
    test_vector3_new!(f32);
    test_vector3_new!(f64);
    test_vector3_new!(i32);
    test_vector3_new!(u32);
    test_vector3_new!(i64);
    test_vector3_new!(u64);
}

#[test]
fn test_vector3_zero() {
    test_vector3_zero!(f32);
    test_vector3_zero!(f64);
    test_vector3_zero!(i32);
    test_vector3_zero!(u32);
    test_vector3_zero!(i64);
    test_vector3_zero!(u64);
}

#[test]
fn test_vector3_default() {
    test_vector3_default!(f32);
    test_vector3_default!(f64);
    test_vector3_default!(i32);
    test_vector3_default!(u32);
    test_vector3_default!(i64);
    test_vector3_default!(u64);
}

#[test]
fn test_vector3_one() {
    test_vector3_one!(f32);
    test_vector3_one!(f64);
    test_vector3_one!(i32);
    test_vector3_one!(u32);
    test_vector3_one!(i64);
    test_vector3_one!(u64);
}

#[test]
fn test_vector3_add() {
    test_vector3_add!(f32);
    test_vector3_add!(f64);
    test_vector3_add!(i32);
    test_vector3_add!(u32);
    test_vector3_add!(i64);
    test_vector3_add!(u64);
}

#[test]
fn test_vector3_sub() {
    test_vector3_sub!(f32);
    test_vector3_sub!(f64);
    test_vector3_sub!(i32);
    test_vector3_sub!(u32);
    test_vector3_sub!(i64);
    test_vector3_sub!(u64);
}

#[test]
fn test_vector3_dot() {
    test_vector3_dot!(f32);
    test_vector3_dot!(f64);
    test_vector3_dot!(i32);
    test_vector3_dot!(u32);
    test_vector3_dot!(i64);
    test_vector3_dot!(u64);
}

#[test]
fn test_vector3_magnitude() {
    test_vector3_magnitude!(f32);
    test_vector3_magnitude!(f64);
    test_vector3_magnitude!(i32);
    test_vector3_magnitude!(u32);
    test_vector3_magnitude!(i64);
    test_vector3_magnitude!(u64);
}

#[test]
fn test_vector3_zero_magnitude() {
    test_vector3_zero_magnitude!(f32);
    test_vector3_zero_magnitude!(f64);
    test_vector3_zero_magnitude!(i32);
    test_vector3_zero_magnitude!(u32);
    test_vector3_zero_magnitude!(i64);
    test_vector3_zero_magnitude!(u64);
}

#[test]
fn test_vector3_scalar_multiplication_signed_types() {
    test_vector3_scalar_multiplication!(f32);
    test_vector3_scalar_multiplication!(f64);
    test_vector3_scalar_multiplication!(i32);
    test_vector3_scalar_multiplication!(i64);
}

#[test]
fn test_vector3_scalar_multiplication_unsigned() {
    // u32 and u64 do not support negative values, so we only test multiplication separately
    let v = Vector3::new(1u32, 2u32, 3u32);
    let result = v * 2u32;
    let expected = Vector3::new(2u32, 4u32, 6u32);
    assert_eq!(result, expected);
    let result_div = v / 2u32;
    let expected_div = Vector3::new(0u32, 1u32, 1u32);
    assert_eq!(result_div, expected_div);

    // for u64
    let v = Vector3::new(1u64, 2u64, 3u64);
    let result = v * 2u64;
    let expected = Vector3::new(2u64, 4u64, 6u64);
    assert_eq!(result, expected);
    let result_div = v / 2u64;
    let expected_div = Vector3::new(0u64, 1u64, 1u64);
    assert_eq!(result_div, expected_div);
}

#[test]
fn test_vector3_distance() {
    test_vector3_distance!(f32);
    test_vector3_distance!(f64);
    test_vector3_distance!(i32);
    test_vector3_distance!(i64);
    // TODO: handle overflow on unsigned types
    test_vector3_distance!(u32);
    test_vector3_distance!(u64);
}

#[test]
fn test_vector3_cross() {
    test_vector3_cross!(f32);
    test_vector3_cross!(f64);
    test_vector3_cross!(i32);
    test_vector3_cross!(u32);
    test_vector3_cross!(i64);
    test_vector3_cross!(u64);
}

#[test]
fn test_vector3_rotate_x_90_deg() {
    test_vector3_rotate!(f32, rotate_x, std::f32::consts::FRAC_PI_2, Vector3::new(1.0, -3.0, 2.0));
    test_vector3_rotate!(f64, rotate_x, std::f64::consts::FRAC_PI_2, Vector3::new(1.0, -3.0, 2.0));
}

#[test]
fn test_vector3_rotate_y_90_deg() {
    test_vector3_rotate!(f32, rotate_y, std::f32::consts::FRAC_PI_2, Vector3::new(-3.0, 2.0, 1.0));
    test_vector3_rotate!(f64, rotate_y, std::f64::consts::FRAC_PI_2, Vector3::new(-3.0, 2.0, 1.0));
}

#[test]
fn test_vector3_rotate_z_90_deg() {
    test_vector3_rotate!(f32, rotate_z, std::f32::consts::FRAC_PI_2, Vector3::new(-2.0, 1.0, 3.0));
    test_vector3_rotate!(f64, rotate_z, std::f64::consts::FRAC_PI_2, Vector3::new(-2.0, 1.0, 3.0));
}

#[test]
fn test_vector3_rotate_axis_90_deg() {
    // for f32
    let v = Vector3::<f32>::new(1.0, 0.0, 0.0);
    let axis = Vector3::new(0.0, 0.0, 1.0);
    let rotated = v.rotate(std::f32::consts::FRAC_PI_2, &axis);
    assert!((rotated.x).abs() < 1e-6);
    assert!((rotated.y - 1.0).abs() < 1e-6);
    assert!((rotated.z).abs() < 1e-6);
    // for f64
    let v = Vector3::<f64>::new(1.0, 0.0, 0.0);
    let axis = Vector3::new(0.0, 0.0, 1.0);
    let rotated = v.rotate(std::f64::consts::FRAC_PI_2, &axis);
    assert!((rotated.x).abs() < 1e-6);
    assert!((rotated.y - 1.0).abs() < 1e-6);
    assert!((rotated.z).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_x_zero() {
    test_vector3_rotate!(f32, rotate_x, 0.0, Vector3::new(1.0, 2.0, 3.0));
    test_vector3_rotate!(f64, rotate_x, 0.0, Vector3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_vector3_rotate_y_zero() {
    test_vector3_rotate!(f32, rotate_y, 0.0, Vector3::new(1.0, 2.0, 3.0));
    test_vector3_rotate!(f64, rotate_y, 0.0, Vector3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_vector3_rotate_z_zero() {
    test_vector3_rotate!(f32, rotate_z, 0.0, Vector3::new(1.0, 2.0, 3.0));
    test_vector3_rotate!(f64, rotate_z, 0.0, Vector3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_vector3_rotate_axis_zero() {
    // for f32
    let v = Vector3::<f32>::new(1.0, 2.0, 3.0);
    let axis = Vector3::new(0.0, 1.0, 0.0);
    let rotated = v.rotate(0.0, &axis);
    assert!((rotated.x - 1.0).abs() < 1e-6);
    assert!((rotated.y - 2.0).abs() < 1e-6);
    assert!((rotated.z - 3.0).abs() < 1e-6);
    // for f64
    let v = Vector3::<f64>::new(1.0, 2.0, 3.0);
    let axis = Vector3::new(0.0, 1.0, 0.0);
    let rotated = v.rotate(0.0, &axis);
    assert!((rotated.x - 1.0).abs() < 1e-6);
    assert!((rotated.y - 2.0).abs() < 1e-6);
    assert!((rotated.z - 3.0).abs() < 1e-6);
}

#[test]
fn test_vector3_normalize() {
    test_vector3_normalize!(f32);
    test_vector3_normalize!(f64);
}

#[test]
fn test_vector3_as_slice() {
    test_vector3_as_slice!(f32);
    test_vector3_as_slice!(f64);
    test_vector3_as_slice!(i32);
    test_vector3_as_slice!(i64);
    test_vector3_as_slice!(u32);
    test_vector3_as_slice!(u64);
}

#[test]
fn test_vector3_as_mut_slice() {
    test_vector3_as_mut_slice!(f32);
    test_vector3_as_mut_slice!(f64);
    test_vector3_as_mut_slice!(i32);
    test_vector3_as_mut_slice!(i64);
    test_vector3_as_mut_slice!(u32);
    test_vector3_as_mut_slice!(u64);
}

#[test]
fn test_vector3_as_ptr_f32() {
    test_vector3_as_ptr!(f32);
    test_vector3_as_ptr!(f64);
    test_vector3_as_ptr!(i32);
    test_vector3_as_ptr!(i64);
    test_vector3_as_ptr!(u32);
    test_vector3_as_ptr!(u64);
}

#[test]
fn test_vector3_as_mut_ptr_f32() {
    test_vector3_as_mut_ptr!(f32);
    test_vector3_as_mut_ptr!(f64);
    test_vector3_as_mut_ptr!(i32);
    test_vector3_as_mut_ptr!(i64);
    test_vector3_as_mut_ptr!(u32);
    test_vector3_as_mut_ptr!(u64);
}
