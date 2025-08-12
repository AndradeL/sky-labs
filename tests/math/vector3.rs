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

#[test]
fn test_vector3_new() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_vector3_add() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);
    let result = v1 + v2;
    assert_eq!(result, Vector3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_vector3_sub() {
    let v1 = Vector3::new(5.0, 7.0, 9.0);
    let v2 = Vector3::new(1.0, 2.0, 3.0);
    let result = v1 - v2;
    assert_eq!(result, Vector3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_vector3_dot() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);
    let result = v1.dot(&v2);
    assert_eq!(result, 32.0);
}

#[test]
fn test_vector3_magnitude() {
    let v = Vector3::new(3.0, 4.0, 0.0);
    assert_eq!(v.magnitude(), 5.0);
}

#[test]
fn test_vector3_zero_magnitude() {
    let v = Vector3::new(0.0, 0.0, 0.0);
    assert_eq!(v.magnitude(), 0.0);
}

#[test]
fn test_vector3_scale() {
    let v = Vector3::new(1.0, -2.0, 3.0);
    let result = v * 2.0;
    assert_eq!(result, Vector3::new(2.0, -4.0, 6.0));
    let result_div = v / 2.0;
    assert_eq!(result_div, Vector3::new(0.5, -1.0, 1.5));
}

#[test]
fn test_vector3_distance() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 6.0, 3.0);
    let dist = v1.distance_to(&v2);
    assert!((dist - 5.0).abs() < 1e-6);
}

#[test]
fn test_vector3_cross() {
    let v1 = Vector3::new(1.0, 0.0, 0.0);
    let v2 = Vector3::new(0.0, 1.0, 0.0);
    let cross = v1.cross(&v2);
    assert_eq!(cross, Vector3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_vector3_rotate_x_90_deg() {
    let v = Vector3::<f64>::new(0.0, 1.0, 0.0);
    let rotated = v.rotate_x(std::f64::consts::FRAC_PI_2);
    assert!((rotated.x).abs() < 1e-6);
    assert!((rotated.y).abs() < 1e-6);
    assert!((rotated.z - 1.0).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_y_90_deg() {
    let v = Vector3::<f64>::new(1.0, 0.0, 0.0);
    let rotated = v.rotate_y(std::f64::consts::FRAC_PI_2);
    assert!((rotated.x).abs() < 1e-6);
    assert!((rotated.y).abs() < 1e-6);
    assert!((rotated.z - 1.0).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_z_90_deg() {
    let v = Vector3::<f64>::new(1.0, 0.0, 0.0);
    let rotated = v.rotate_z(std::f64::consts::FRAC_PI_2);
    assert!((rotated.x).abs() < 1e-6);
    assert!((rotated.y - 1.0).abs() < 1e-6);
    assert!((rotated.z).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_axis_90_deg() {
    let v = Vector3::<f64>::new(1.0, 0.0, 0.0);
    let axis = Vector3::new(0.0, 0.0, 1.0);
    let rotated = v.rotate(std::f64::consts::FRAC_PI_2, &axis);
    assert!((rotated.x).abs() < 1e-6);
    assert!((rotated.y - 1.0).abs() < 1e-6);
    assert!((rotated.z).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_x_zero() {
    let v = Vector3::<f64>::new(1.0, 2.0, 3.0);
    let rotated = v.rotate_x(0.0);
    assert!((rotated.x - 1.0).abs() < 1e-6);
    assert!((rotated.y - 2.0).abs() < 1e-6);
    assert!((rotated.z - 3.0).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_y_zero() {
    let v = Vector3::<f32>::new(1.0, 2.0, 3.0);
    let rotated = v.rotate_y(0.0);
    assert!((rotated.x - 1.0).abs() < 1e-6);
    assert!((rotated.y - 2.0).abs() < 1e-6);
    assert!((rotated.z - 3.0).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_z_zero() {
    let v = Vector3::<f64>::new(1.0, 2.0, 3.0);
    let rotated = v.rotate_z(0.0);
    assert!((rotated.x - 1.0).abs() < 1e-6);
    assert!((rotated.y - 2.0).abs() < 1e-6);
    assert!((rotated.z - 3.0).abs() < 1e-6);
}

#[test]
fn test_vector3_rotate_axis_zero() {
    let v = Vector3::<f64>::new(1.0, 2.0, 3.0);
    let axis = Vector3::new(0.0, 1.0, 0.0);
    let rotated = v.rotate(0.0, &axis);
    assert!((rotated.x - (-1.0)).abs() < 1e-6);
    assert!((rotated.y - 2.0).abs() < 1e-6);
    assert!((rotated.z - (-3.0)).abs() < 1e-6);
}
#[test]
fn test_vector3_as_slice_f32() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let slice = v.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(std::mem::size_of_val(slice), 12); // 3 * size of f32
    assert_eq!(slice[0], 1.0f32);
    assert_eq!(slice[1], 2.0f32);
    assert_eq!(slice[2], 3.0f32);
}

#[test]
fn test_vector3_as_slice_f64() {
    let v = Vector3::new(1.0f64, 2.0f64, 3.0f64);
    let slice = v.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(std::mem::size_of_val(slice), 24); // 3 * size of f64
    assert_eq!(slice[0], 1.0f64);
    assert_eq!(slice[1], 2.0f64);
    assert_eq!(slice[2], 3.0f64);
}

#[test]
fn test_vector3_as_ptr_f32() {
    let v = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    unsafe {
        let ptr = v.as_ptr();
        assert_eq!(*ptr.offset(0), 4.0f32);
        assert_eq!(*ptr.offset(1), 5.0f32);
        assert_eq!(*ptr.offset(2), 6.0f32);
    }
}

#[test]
fn test_vector3_as_ptr_f64() {
    let v = Vector3::new(4.0f64, 5.0f64, 6.0f64);
    unsafe {
        let ptr = v.as_ptr();
        assert_eq!(*ptr.offset(0), 4.0f64);
        assert_eq!(*ptr.offset(1), 5.0f64);
        assert_eq!(*ptr.offset(2), 6.0f64);
    }
}

#[test]
fn test_vector3_as_mut_slice_f32() {
    let mut v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    {
        let slice = v.as_mut_slice();
        assert_eq!(slice.len(), 3);
        slice[0] = 10.0f32;
        slice[1] = 20.0f32;
        slice[2] = 30.0f32;
    }
    assert_eq!(v.x, 10.0f32);
    assert_eq!(v.y, 20.0f32);
    assert_eq!(v.z, 30.0f32);
}

#[test]
fn test_vector3_as_mut_slice_f64() {
    let mut v = Vector3::new(1.0f64, 2.0f64, 3.0f64);
    {
        let slice = v.as_mut_slice();
        assert_eq!(slice.len(), 3);
        slice[0] = 100.0f64;
        slice[1] = 200.0f64;
        slice[2] = 300.0f64;
    }
    assert_eq!(v.x, 100.0f64);
    assert_eq!(v.y, 200.0f64);
    assert_eq!(v.z, 300.0f64);
}

#[test]
fn test_vector3_as_mut_ptr_f32() {
    let mut v = Vector3::new(7.0f32, 8.0f32, 9.0f32);
    unsafe {
        let ptr = v.as_mut_ptr();
        *ptr.offset(0) = 70.0f32;
        *ptr.offset(1) = 80.0f32;
        *ptr.offset(2) = 90.0f32;
    }
    assert_eq!(v.x, 70.0f32);
    assert_eq!(v.y, 80.0f32);
    assert_eq!(v.z, 90.0f32);
}

#[test]
fn test_vector3_as_mut_ptr_f64() {
    let mut v = Vector3::new(7.0f64, 8.0f64, 9.0f64);
    unsafe {
        let ptr = v.as_mut_ptr();
        *ptr.offset(0) = 700.0f64;
        *ptr.offset(1) = 800.0f64;
        *ptr.offset(2) = 900.0f64;
    }
    assert_eq!(v.x, 700.0f64);
    assert_eq!(v.y, 800.0f64);
    assert_eq!(v.z, 900.0f64);
}
