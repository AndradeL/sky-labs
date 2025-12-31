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

use sky_labs::math::{Vector3, Vector4};

macro_rules! test_vector4_new {
    ($type:ty) => {
        let v = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        assert_eq!(v.x, 1 as $type);
        assert_eq!(v.y, 2 as $type);
        assert_eq!(v.z, 3 as $type);
        assert_eq!(v.w, 4 as $type);
    };
}

macro_rules! test_vector4_zero {
    ($type:ty) => {
        let v = Vector4::<$type>::zero();
        assert_eq!(v.x, 0 as $type);
        assert_eq!(v.y, 0 as $type);
        assert_eq!(v.z, 0 as $type);
        assert_eq!(v.w, 0 as $type);
    };
}

macro_rules! test_vector4_default {
    ($type:ty) => {
        let v: Vector4<$type> = Default::default();
        assert_eq!(v.x, 0 as $type);
        assert_eq!(v.y, 0 as $type);
        assert_eq!(v.z, 0 as $type);
        assert_eq!(v.w, 0 as $type);
    };
}

macro_rules! test_vector4_one {
    ($type:ty) => {
        let v = Vector4::<$type>::one();
        assert_eq!(v.x, 1 as $type);
        assert_eq!(v.y, 1 as $type);
        assert_eq!(v.z, 1 as $type);
        assert_eq!(v.w, 1 as $type);
    };
}

macro_rules! test_vector4_add {
    ($type:ty) => {
        let v1 = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        let v2 = Vector4::<$type>::new(4 as $type, 5 as $type, 6 as $type, 7 as $type);
        let expected = Vector4::<$type>::new(5 as $type, 7 as $type, 9 as $type, 11 as $type);
        let result = v1 + v2;
        assert_eq!(result, expected);
    };
}

macro_rules! test_vector4_sub {
    ($type:ty) => {
        let v1 = Vector4::<$type>::new(5 as $type, 7 as $type, 9 as $type, 11 as $type);
        let v2 = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        let expected = Vector4::<$type>::new(4 as $type, 5 as $type, 6 as $type, 7 as $type);
        let result = v1 - v2;
        assert_eq!(result, expected);
    };
}

macro_rules! test_vector4_dot {
    ($type:ty) => {
        let v1 = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        let v2 = Vector4::<$type>::new(4 as $type, 5 as $type, 6 as $type, 7 as $type);
        let expected = 60 as $type; // 1*4 + 2*5 + 3*6 + 4*7 = 4 + 10 + 18 + 28 = 60
        let result = v1.dot(&v2);
        assert_eq!(result, expected);
    };
}

macro_rules! test_vector4_neg {
    ($type:ty) => {
        let v = Vector4::<$type>::new(1 as $type, -2 as $type, 3 as $type, -4 as $type);
        let result = -v;
        assert_eq!(result.x, -v.x);
        assert_eq!(result.y, -v.y);
        assert_eq!(result.z, -v.z);
        assert_eq!(result.w, -v.w);
    };
}

macro_rules! test_vector4_scalar_multiplication {
    ($type:ty) => {
        let v = Vector4::<$type>::new(1 as $type, -2 as $type, 3 as $type, -4 as $type);
        let result = v * 2 as $type;
        let expected = Vector4::<$type>::new(2 as $type, -4 as $type, 6 as $type, -8 as $type);
        assert_eq!(result, expected);
        let result_div = v / 2 as $type;
        let expected_div =
            Vector4::<$type>::new(0.5 as $type, -1.0 as $type, 1.5 as $type, -2.0 as $type);
        assert_eq!(result_div, expected_div);
    };
}

macro_rules! test_vector4_index {
    ($type:ty) => {
        let mut v = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        assert_eq!(v[0], 1 as $type);
        assert_eq!(v[1], 2 as $type);
        assert_eq!(v[2], 3 as $type);
        assert_eq!(v[3], 4 as $type);
        v[0] = 10 as $type;
        v[1] = 20 as $type;
        v[2] = 30 as $type;
        v[3] = 40 as $type;
        assert_eq!(v.x, 10 as $type);
        assert_eq!(v.y, 20 as $type);
        assert_eq!(v.z, 30 as $type);
        assert_eq!(v.w, 40 as $type);
    };
}

macro_rules! test_vector4_from_array_and_to_array {
    ($type:ty) => {
        let arr = [1 as $type, 2 as $type, 3 as $type, 4 as $type];
        let v = Vector4::<$type>::from_array(arr);
        let out = v.to_array();
        assert_eq!(out, arr);
    };
}

macro_rules! test_vector4_from_slice {
    ($type:ty) => {
        let slice: [$type; 4] = [1 as $type, 2 as $type, 3 as $type, 4 as $type];
        let v = Vector4::<$type>::from_slice(&slice);
        let expected = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        assert_eq!(v, expected);
    };
}

macro_rules! test_vector4_as_slice {
    ($type:ty) => {
        let v = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        let slice = v.as_slice();
        assert_eq!(slice.len(), 4);
        assert_eq!(
            std::mem::size_of_val(slice),
            4 * std::mem::size_of::<$type>()
        );
        assert_eq!(slice[0], 1 as $type);
        assert_eq!(slice[1], 2 as $type);
        assert_eq!(slice[2], 3 as $type);
        assert_eq!(slice[3], 4 as $type);
    };
}

macro_rules! test_vector4_as_mut_slice {
    ($type:ty) => {
        let mut v = Vector4::<$type>::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
        {
            let slice = v.as_mut_slice();
            assert_eq!(slice.len(), 4);
            assert_eq!(
                std::mem::size_of_val(slice),
                4 * std::mem::size_of::<$type>()
            );
            slice[0] = 10 as $type;
            slice[1] = 20 as $type;
            slice[2] = 30 as $type;
            slice[3] = 40 as $type;
        }
        assert_eq!(v.x, 10 as $type);
        assert_eq!(v.y, 20 as $type);
        assert_eq!(v.z, 30 as $type);
        assert_eq!(v.w, 40 as $type);
    };
}

macro_rules! test_vector4_as_ptr {
    ($type:ty) => {
        let v = Vector4::<$type>::new(4 as $type, 5 as $type, 6 as $type, 7 as $type);
        unsafe {
            let ptr = v.as_ptr();
            assert_eq!(*ptr.offset(0), 4 as $type);
            assert_eq!(*ptr.offset(1), 5 as $type);
            assert_eq!(*ptr.offset(2), 6 as $type);
            assert_eq!(*ptr.offset(3), 7 as $type);
        }
    };
}

macro_rules! test_vector4_as_mut_ptr {
    ($type:ty) => {
        let mut v = Vector4::<$type>::new(7 as $type, 8 as $type, 9 as $type, 10 as $type);
        unsafe {
            let ptr = v.as_mut_ptr();
            *ptr.offset(0) = 70 as $type;
            *ptr.offset(1) = 80 as $type;
            *ptr.offset(2) = 90 as $type;
            *ptr.offset(3) = 100 as $type;
        }
        assert_eq!(v.x, 70 as $type);
        assert_eq!(v.y, 80 as $type);
        assert_eq!(v.z, 90 as $type);
        assert_eq!(v.w, 100 as $type);
    };
}

macro_rules! test_vector4_from_vector3 {
    ($type:ty) => {
        let v3 = Vector3::<$type>::new(1 as $type, 2 as $type, 3 as $type);
        let v4 = Vector4::<$type>::from_vector3(&v3, 4 as $type);
        assert_eq!(v4.x, 1 as $type);
        assert_eq!(v4.y, 2 as $type);
        assert_eq!(v4.z, 3 as $type);
        assert_eq!(v4.w, 4 as $type);
    };
}

#[test]
fn test_vector4_new() {
    test_vector4_new!(f32);
    test_vector4_new!(f64);
    test_vector4_new!(i32);
    test_vector4_new!(u32);
    test_vector4_new!(i64);
    test_vector4_new!(u64);
}

#[test]
fn test_vector4_zero() {
    test_vector4_zero!(f32);
    test_vector4_zero!(f64);
    test_vector4_zero!(i32);
    test_vector4_zero!(u32);
    test_vector4_zero!(i64);
    test_vector4_zero!(u64);
}

#[test]
fn test_vector4_default() {
    test_vector4_default!(f32);
    test_vector4_default!(f64);
    test_vector4_default!(i32);
    test_vector4_default!(u32);
    test_vector4_default!(i64);
    test_vector4_default!(u64);
}

#[test]
fn test_vector4_one() {
    test_vector4_one!(f32);
    test_vector4_one!(f64);
    test_vector4_one!(i32);
    test_vector4_one!(u32);
    test_vector4_one!(i64);
    test_vector4_one!(u64);
}

#[test]
fn test_vector4_add() {
    test_vector4_add!(f32);
    test_vector4_add!(f64);
    test_vector4_add!(i32);
    test_vector4_add!(u32);
    test_vector4_add!(i64);
    test_vector4_add!(u64);
}

#[test]
fn test_vector4_sub() {
    test_vector4_sub!(f32);
    test_vector4_sub!(f64);
    test_vector4_sub!(i32);
    test_vector4_sub!(u32);
    test_vector4_sub!(i64);
    test_vector4_sub!(u64);
}

#[test]
fn test_vector4_dot() {
    test_vector4_dot!(f32);
    test_vector4_dot!(f64);
    test_vector4_dot!(i32);
    test_vector4_dot!(u32);
    test_vector4_dot!(i64);
    test_vector4_dot!(u64);
}

#[test]
fn test_vector4_neg() {
    test_vector4_neg!(f32);
    test_vector4_neg!(f64);
    test_vector4_neg!(i32);
    test_vector4_neg!(i64);
}

#[test]
fn test_vector4_scalar_multiplication_signed_types() {
    test_vector4_scalar_multiplication!(f32);
    test_vector4_scalar_multiplication!(f64);
    test_vector4_scalar_multiplication!(i32);
    test_vector4_scalar_multiplication!(i64);
}

#[test]
fn test_vector4_scalar_multiplication_unsigned() {
    // u32 and u64 do not support negative values, so we only test multiplication separately
    let v = Vector4::new(1u32, 2u32, 3u32, 4u32);
    let result = v * 2u32;
    let expected = Vector4::new(2u32, 4u32, 6u32, 8u32);
    assert_eq!(result, expected);
    let result_div = v / 2u32;
    let expected_div = Vector4::new(0u32, 1u32, 1u32, 2u32);
    assert_eq!(result_div, expected_div);

    // for u64
    let v = Vector4::new(1u64, 2u64, 3u64, 4u64);
    let result = v * 2u64;
    let expected = Vector4::new(2u64, 4u64, 6u64, 8u64);
    assert_eq!(result, expected);
    let result_div = v / 2u64;
    let expected_div = Vector4::new(0u64, 1u64, 1u64, 2u64);
    assert_eq!(result_div, expected_div);
}

#[test]
fn test_vector4_assign_ops() {
    let mut v = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    v += Vector4::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(v, Vector4::new(2.0, 3.0, 4.0, 5.0));

    v -= Vector4::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(v, Vector4::new(1.0, 2.0, 3.0, 4.0));

    v *= 2.0f32;
    assert_eq!(v, Vector4::new(2.0, 4.0, 6.0, 8.0));

    v /= 2.0f32;
    assert_eq!(v, Vector4::new(1.0, 2.0, 3.0, 4.0));
}

#[test]
fn test_vector4_indexing() {
    test_vector4_index!(f32);
    test_vector4_index!(f64);
    test_vector4_index!(i32);
    test_vector4_index!(i64);
    test_vector4_index!(u32);
    test_vector4_index!(u64);
}

#[test]
fn test_vector4_from_array_and_to_array() {
    test_vector4_from_array_and_to_array!(f32);
    test_vector4_from_array_and_to_array!(f64);
    test_vector4_from_array_and_to_array!(i32);
    test_vector4_from_array_and_to_array!(i64);
    test_vector4_from_array_and_to_array!(u32);
    test_vector4_from_array_and_to_array!(u64);
}

#[test]
fn test_vector4_from_slice() {
    test_vector4_from_slice!(f32);
    test_vector4_from_slice!(f64);
    test_vector4_from_slice!(i32);
    test_vector4_from_slice!(i64);
    test_vector4_from_slice!(u32);
    test_vector4_from_slice!(u64);
}

#[test]
fn test_vector4_as_slice_and_mut() {
    test_vector4_as_slice!(f32);
    test_vector4_as_slice!(f64);
    test_vector4_as_slice!(i32);
    test_vector4_as_slice!(i64);
    test_vector4_as_slice!(u32);
    test_vector4_as_slice!(u64);

    test_vector4_as_mut_slice!(f32);
    test_vector4_as_mut_slice!(f64);
    test_vector4_as_mut_slice!(i32);
    test_vector4_as_mut_slice!(i64);
    test_vector4_as_mut_slice!(u32);
    test_vector4_as_mut_slice!(u64);
}

#[test]
fn test_vector4_as_ptr_and_mut_ptr() {
    test_vector4_as_ptr!(f32);
    test_vector4_as_ptr!(f64);
    test_vector4_as_ptr!(i32);
    test_vector4_as_ptr!(i64);
    test_vector4_as_ptr!(u32);
    test_vector4_as_ptr!(u64);

    test_vector4_as_mut_ptr!(f32);
    test_vector4_as_mut_ptr!(f64);
    test_vector4_as_mut_ptr!(i32);
    test_vector4_as_mut_ptr!(i64);
    test_vector4_as_mut_ptr!(u32);
    test_vector4_as_mut_ptr!(u64);
}

#[test]
fn test_vector4_from_vector3() {
    test_vector4_from_vector3!(f32);
    test_vector4_from_vector3!(f64);
    test_vector4_from_vector3!(i32);
    test_vector4_from_vector3!(u32);
    test_vector4_from_vector3!(i64);
    test_vector4_from_vector3!(u64);
}
