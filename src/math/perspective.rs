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

use super::Matrix4x4;

pub fn perspective_f32(
    horizontal_fov: f32,
    aspect_ratio: f32,
    near_field: f32,
    far_field: f32,
) -> Matrix4x4<f32> {
    let focal_length = 1.0 / (horizontal_fov / 2.0).tan();
    let range_inv = 1.0 / (far_field - near_field);
    let far_range = far_field * range_inv;

    Matrix4x4::from_mat([
        [focal_length / aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, focal_length, 0.0, 0.0],
        [0.0, 0.0, far_range, -1.0 * near_field * far_range],
        [0.0, 0.0, 1.0, 0.0],
    ])
}

pub fn perspective_f64(
    horizontal_fov: f64,
    aspect_ratio: f64,
    near_field: f64,
    far_field: f64,
) -> Matrix4x4<f64> {
    let focal_length = 1.0 / (horizontal_fov / 2.0).tan();
    let range_inv = 1.0 / (far_field - near_field);
    let far_range = far_field * range_inv;

    Matrix4x4::from_mat([
        [focal_length / aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, focal_length, 0.0, 0.0],
        [0.0, 0.0, far_range, -1.0 * near_field * far_range],
        [0.0, 0.0, 1.0, 0.0],
    ])
}
