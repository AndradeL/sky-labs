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

use sky_labs::math::Vector2;

#[test]
fn test_vector2_addition_f32() {
    let v1 = Vector2::new(1.0f32, 2.0f32);
    let v2 = Vector2::new(3.0f32, 4.0f32);
    let result = v1 + v2;
    assert_eq!(result, Vector2::new(4.0f32, 6.0f32));
}

#[test]
fn test_vector2_addition_f64() {
    let v1 = Vector2::new(1.0f64, 2.0f64);
    let v2 = Vector2::new(3.0f64, 4.0f64);
    let result = v1 + v2;
    assert_eq!(result, Vector2::new(4.0f64, 6.0f64));
}

#[test]
fn test_vector2_addition_i32() {
    let v1 = Vector2::new(1i32, 2i32);
    let v2 = Vector2::new(3i32, 4i32);
    let result = v1 + v2;
    assert_eq!(result, Vector2::new(4i32, 6i32));
}

#[test]
fn test_vector2_addition_i64() {
    let v1 = Vector2::new(1i64, 2i64);
    let v2 = Vector2::new(3i64, 4i64);
    let result = v1 + v2;
    assert_eq!(result, Vector2::new(4i64, 6i64));
}

#[test]
fn test_vector2_addition_u32() {
    let v1 = Vector2::new(1u32, 2u32);
    let v2 = Vector2::new(3u32, 4u32);
    let result = v1 + v2;
    assert_eq!(result, Vector2::new(4u32, 6u32));
}

#[test]
fn test_vector2_addition_u64() {
    let v1 = Vector2::new(1u64, 2u64);
    let v2 = Vector2::new(3u64, 4u64);
    let result = v1 + v2;
    assert_eq!(result, Vector2::new(4u64, 6u64));
}

// Similar tests can be created for subtraction, multiplication, division, magnitude, normalize, and dot product for each type.
#[test]
fn test_vector2_subtraction_f32() {
    let v1 = Vector2::new(5.0f32, 6.0f32);
    let v2 = Vector2::new(2.0f32, 3.0f32);
    let result = v1 - v2;
    assert_eq!(result, Vector2::new(3.0f32, 3.0f32));
}

#[test]
fn test_vector2_subtraction_f64() {
    let v1 = Vector2::new(5.0f64, 6.0f64);
    let v2 = Vector2::new(2.0f64, 3.0f64);
    let result = v1 - v2;
    assert_eq!(result, Vector2::new(3.0f64, 3.0f64));
}

#[test]
fn test_vector2_subtraction_i32() {
    let v1 = Vector2::new(5i32, 6i32);
    let v2 = Vector2::new(2i32, 3i32);
    let result = v1 - v2;
    assert_eq!(result, Vector2::new(3i32, 3i32));
}

#[test]
fn test_vector2_subtraction_i64() {
    let v1 = Vector2::new(5i64, 6i64);
    let v2 = Vector2::new(2i64, 3i64);
    let result = v1 - v2;
    assert_eq!(result, Vector2::new(3i64, 3i64));
}

#[test]
fn test_vector2_subtraction_u32() {
    let v1 = Vector2::new(5u32, 6u32);
    let v2 = Vector2::new(2u32, 3u32);
    let result = v1 - v2;
    assert_eq!(result, Vector2::new(3u32, 3u32));
}

#[test]
fn test_vector2_subtraction_u64() {
    let v1 = Vector2::new(5u64, 6u64);
    let v2 = Vector2::new(2u64, 3u64);
    let result = v1 - v2;
    assert_eq!(result, Vector2::new(3u64, 3u64));
}

#[test]
fn test_vector2_multiplication_f32() {
    let v = Vector2::new(2.0f32, 3.0f32);
    let scalar = 2.0f32;
    let result = v * scalar;
    assert_eq!(result, Vector2::new(4.0f32, 6.0f32));
}

#[test]
fn test_vector2_multiplication_f64() {
    let v = Vector2::new(2.0f64, 3.0f64);
    let scalar = 2.0f64;
    let result = v * scalar;
    assert_eq!(result, Vector2::new(4.0f64, 6.0f64));
}

#[test]
fn test_vector2_multiplication_i32() {
    let v = Vector2::new(2i32, 3i32);
    let scalar = 2i32;
    let result = v * scalar;
    assert_eq!(result, Vector2::new(4i32, 6i32));
}

#[test]
fn test_vector2_multiplication_i64() {
    let v = Vector2::new(2i64, 3i64);
    let scalar = 2i64;
    let result = v * scalar;
    assert_eq!(result, Vector2::new(4i64, 6i64));
}

#[test]
fn test_vector2_multiplication_u32() {
    let v = Vector2::new(2u32, 3u32);
    let scalar = 2u32;
    let result = v * scalar;
    assert_eq!(result, Vector2::new(4u32, 6u32));
}

#[test]
fn test_vector2_multiplication_u64() {
    let v = Vector2::new(2u64, 3u64);
    let scalar = 2u64;
    let result = v * scalar;
    assert_eq!(result, Vector2::new(4u64, 6u64));
}

#[test]
fn test_vector2_division_f32() {
    let v = Vector2::new(4.0f32, 6.0f32);
    let scalar = 2.0f32;
    let result = v / scalar;
    assert_eq!(result, Vector2::new(2.0f32, 3.0f32));
}

#[test]
fn test_vector2_division_f64() {
    let v = Vector2::new(4.0f64, 6.0f64);
    let scalar = 2.0f64;
    let result = v / scalar;
    assert_eq!(result, Vector2::new(2.0f64, 3.0f64));
}

#[test]
fn test_vector2_division_i32() {
    let v = Vector2::new(4i32, 6i32);
    let scalar = 2i32;
    let result = v / scalar;
    assert_eq!(result, Vector2::new(2i32, 3i32));
}

#[test]
fn test_vector2_division_i64() {
    let v = Vector2::new(4i64, 6i64);
    let scalar = 2i64;
    let result = v / scalar;
    assert_eq!(result, Vector2::new(2i64, 3i64));
}

#[test]
fn test_vector2_division_u32() {
    let v = Vector2::new(4u32, 6u32);
    let scalar = 2u32;
    let result = v / scalar;
    assert_eq!(result, Vector2::new(2u32, 3u32));
}

#[test]
fn test_vector2_division_u64() {
    let v = Vector2::new(4u64, 6u64);
    let scalar = 2u64;
    let result = v / scalar;
    assert_eq!(result, Vector2::new(2u64, 3u64));
}

#[test]
fn test_vector2_magnitude_f32() {
    let v = Vector2::new(3.0f32, 4.0f32);
    let result = v.magnitude();
    assert_eq!(result, 5.0f64);
}

#[test]
fn test_vector2_magnitude_f64() {
    let v = Vector2::new(3.0f64, 4.0f64);
    let result = v.magnitude();
    assert_eq!(result, 5.0f64);
}

#[test]
fn test_vector2_magnitude_i32() {
    let v = Vector2::new(3i32, 4i32);
    let result = v.magnitude();
    assert_eq!(result, 5.0f64);
}

#[test]
fn test_vector2_magnitude_i64() {
    let v = Vector2::new(3i64, 4i64);
    let result = v.magnitude();
    assert_eq!(result, 5.0f64);
}

#[test]
fn test_vector2_magnitude_u32() {
    let v = Vector2::new(3u32, 4u32);
    let result = v.magnitude();
    assert_eq!(result, 5.0f64);
}

#[test]
fn test_vector2_magnitude_u64() {
    let v = Vector2::new(3u64, 4u64);
    let result = v.magnitude();
    assert_eq!(result, 5.0f64);
}

#[test]
fn test_vector2_normalize_f32() {
    let v = Vector2::new(3.0f32, 4.0f32);
    let result = v.normalize();
    assert_eq!(result, Vector2::new(0.6f32, 0.8f32));
}

#[test]
fn test_vector2_normalize_f64() {
    let v = Vector2::new(3.0f64, 4.0f64);
    let result = v.normalize();
    assert_eq!(result, Vector2::new(0.6f64, 0.8f64));
}

#[test]
fn test_vector2_dot_product_f32() {
    let v1 = Vector2::new(1.0f32, 2.0f32);
    let v2 = Vector2::new(3.0f32, 4.0f32);
    let result = v1.dot(v2);
    assert_eq!(result, 11.0f32);
}

#[test]
fn test_vector2_dot_product_f64() {
    let v1 = Vector2::new(1.0f64, 2.0f64);
    let v2 = Vector2::new(3.0f64, 4.0f64);
    let result = v1.dot(v2);
    assert_eq!(result, 11.0f64);
}
