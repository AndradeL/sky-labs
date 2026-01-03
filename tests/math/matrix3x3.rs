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

use sky_labs::math::Matrix3x3;
use sky_labs::math::Vector3;

macro_rules! assert_eq_mat {
    ($type:ty, $res:expr, $exp:expr) => {
        let eps = <$type>::EPSILON;
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    ($res[i][j] - $exp[i][j]).abs() <= eps,
                    "{}[{},{}] = {}, {}[{},{}] = {}",
                    stringify!($res),
                    i,
                    j,
                    $res[i][j],
                    stringify!($exp),
                    i,
                    j,
                    $exp[i][j]
                );
            }
        }
    };
}

macro_rules! test_matrix3x3_identity {
    ($type:ty) => {
        let identity = Matrix3x3::<$type>::identity();
        assert_eq!(identity[(0, 0)], 1 as $type);
        assert_eq!(identity[(1, 1)], 1 as $type);
        assert_eq!(identity[(2, 2)], 1 as $type);
        assert_eq!(identity[(0, 1)], 0 as $type);
        assert_eq!(identity[(0, 2)], 0 as $type);
        assert_eq!(identity[(1, 0)], 0 as $type);
        assert_eq!(identity[(1, 2)], 0 as $type);
        assert_eq!(identity[(2, 0)], 0 as $type);
        assert_eq!(identity[(2, 1)], 0 as $type);
    };
}

macro_rules! test_matrix3x3_addition {
    ($type:ty) => {
        let a = Matrix3x3::<$type>::from_mat([
            [1 as $type, 2 as $type, 3 as $type],
            [4 as $type, 5 as $type, 6 as $type],
            [7 as $type, 8 as $type, 9 as $type],
        ]);
        let b = Matrix3x3::<$type>::from_mat([
            [9 as $type, 8 as $type, 7 as $type],
            [6 as $type, 5 as $type, 4 as $type],
            [3 as $type, 2 as $type, 1 as $type],
        ]);
        let result = a + b;
        assert_eq!(result[(0, 0)], 10 as $type);
        assert_eq!(result[(0, 1)], 10 as $type);
        assert_eq!(result[(0, 2)], 10 as $type);
        assert_eq!(result[(1, 0)], 10 as $type);
        assert_eq!(result[(1, 1)], 10 as $type);
        assert_eq!(result[(1, 2)], 10 as $type);
        assert_eq!(result[(2, 0)], 10 as $type);
        assert_eq!(result[(2, 1)], 10 as $type);
        assert_eq!(result[(2, 2)], 10 as $type);
    };
}

macro_rules! test_matrix3x3_multiplication {
    ($type:ty, $eps:expr) => {
        let a = Matrix3x3::<$type>::from_mat([
            [1 as $type, 2 as $type, 3 as $type],
            [0 as $type, 1 as $type, 4 as $type],
            [5 as $type, 6 as $type, 0 as $type],
        ]);
        let b = Matrix3x3::<$type>::from_mat([
            [7 as $type, 8 as $type, 9 as $type],
            [2 as $type, 3 as $type, 4 as $type],
            [1 as $type, 0 as $type, 0 as $type],
        ]);
        let result = a * b;
        let expected = Matrix3x3::<$type>::from_mat([
            [14 as $type, 14 as $type, 17 as $type],
            [6 as $type, 3 as $type, 4 as $type],
            [47 as $type, 58 as $type, 69 as $type],
        ]);
        assert_eq!(result, expected);
    };
}

macro_rules! test_matrix3x3_transpose {
    ($type:ty) => {
        let m = Matrix3x3::<$type>::from_mat([
            [1 as $type, 2 as $type, 3 as $type],
            [4 as $type, 5 as $type, 6 as $type],
            [7 as $type, 8 as $type, 9 as $type],
        ]);
        let result = m.transpose();
        let expected = Matrix3x3::<$type>::from_mat([
            [1 as $type, 4 as $type, 7 as $type],
            [2 as $type, 5 as $type, 8 as $type],
            [3 as $type, 6 as $type, 9 as $type],
        ]);
        assert_eq!(result, expected);
    };
}

macro_rules! test_matrix3x3_determinant {
    ($type:ty, $eps:expr) => {
        let m = Matrix3x3::<$type>::from_mat([
            [6 as $type, 1 as $type, 1 as $type],
            [4 as $type, -2 as $type, 5 as $type],
            [2 as $type, 8 as $type, 7 as $type],
        ]);
        let det = m.determinant();
        let expected = -306 as $type;
        if <$type>::default() == 0 as $type {
            // integer types
            assert_eq!(det, expected as $type);
        } else {
            // float types
            assert!((det as $type - expected).abs() < $eps);
        }
    };
}

macro_rules! test_matrix3x3_inverse_identity {
    ($type:ty, $eps:expr) => {
        let identity = Matrix3x3::<$type>::identity();
        let inv = identity.inverse().unwrap();
        for i in 0..3 {
            for j in 0..3 {
                let expected = identity[(i, j)];
                if <$type>::default() == 0 as $type {
                    // integer types
                    assert_eq!(inv[(i, j)], expected);
                } else {
                    // float types
                    assert!((inv[(i, j)] as f64 - expected as f64).abs() < $eps);
                }
            }
        }
    };
}

macro_rules! test_matrix3x3_inverse_known_matrix {
    ($type:ty, $eps:expr) => {
        let m = Matrix3x3::<$type>::from_mat([
            [1 as $type, 2 as $type, 3 as $type],
            [0 as $type, 1 as $type, 4 as $type],
            [5 as $type, 6 as $type, 0 as $type],
        ]);
        let inv = m.inverse().unwrap();
        let prod = m * inv;
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1 as $type } else { 0 as $type };
                if <$type>::default() == 0 as $type {
                    // integer types
                    assert_eq!(prod[(i, j)], expected);
                } else {
                    // float types
                    assert!(
                        (prod[(i, j)] as f64 - expected as f64).abs() < $eps,
                        "prod[{},{}] = {}",
                        i,
                        j,
                        prod[(i, j)]
                    );
                }
            }
        }
    };
}

macro_rules! test_matrix3x3_inverse_non_invertible {
    ($type:ty) => {
        let m = Matrix3x3::<$type>::from_mat([
            [1 as $type, 2 as $type, 3 as $type],
            [2 as $type, 4 as $type, 6 as $type],
            [3 as $type, 6 as $type, 9 as $type],
        ]);
        assert!(m.inverse().is_none());
    };
}

macro_rules! test_matrix3x3_make_rotation {
    ($type:ty, $rot:ident, $expected:expr, $rad:expr) => {
        let result = Matrix3x3::<$type>::$rot($rad);
        let expected = $expected;
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (result[(i, j)] - expected[(i, j)]).abs() < <$type>::EPSILON,
                    "result[{},{}] = {}, expected = {}",
                    i,
                    j,
                    result[(i, j)],
                    expected[(i, j)]
                );
            }
        }
    };
}

#[test]
fn test_matrix3x3_identity_all_types() {
    test_matrix3x3_identity!(i32);
    test_matrix3x3_identity!(i64);
    test_matrix3x3_identity!(f32);
    test_matrix3x3_identity!(f64);
}

#[test]
fn test_matrix3x3_addition_all_types() {
    test_matrix3x3_addition!(i32);
    test_matrix3x3_addition!(i64);
    test_matrix3x3_addition!(f32);
    test_matrix3x3_addition!(f64);
}

#[test]
fn test_matrix3x3_multiplication_all_types() {
    test_matrix3x3_multiplication!(i32, 1e-6);
    test_matrix3x3_multiplication!(i64, 1e-6);
    test_matrix3x3_multiplication!(f32, 1e-6);
    test_matrix3x3_multiplication!(f64, 1e-12);
}

#[test]
fn test_matrix3x3_transpose_all_types() {
    test_matrix3x3_transpose!(i32);
    test_matrix3x3_transpose!(i64);
    test_matrix3x3_transpose!(f32);
    test_matrix3x3_transpose!(f64);
}

#[test]
fn test_matrix3x3_determinant_all_types() {
    test_matrix3x3_determinant!(i32, 0);
    test_matrix3x3_determinant!(i64, 0);
    test_matrix3x3_determinant!(f32, f32::EPSILON);
    test_matrix3x3_determinant!(f64, f64::EPSILON);
}

#[test]
fn test_matrix3x3_inverse_identity_all_types() {
    test_matrix3x3_inverse_identity!(i32, 0.0);
    test_matrix3x3_inverse_identity!(i64, 0.0);
    test_matrix3x3_inverse_identity!(f32, 1e-6);
    test_matrix3x3_inverse_identity!(f64, 1e-12);
}

#[test]
fn test_matrix3x3_inverse_known_matrix_all_types() {
    test_matrix3x3_inverse_known_matrix!(f32, 1e-6);
    test_matrix3x3_inverse_known_matrix!(f64, 1e-12);
    // Integer types are not tested here because their inverse may not exist or be exact
}

#[test]
fn test_matrix3x3_inverse_non_invertible_all_types() {
    test_matrix3x3_inverse_non_invertible!(f32);
    test_matrix3x3_inverse_non_invertible!(f64);
    // Integer types are not tested here because their inverse may not exist or be exact
}

#[test]
fn test_matrix3x3_make_rotation_z() {
    test_matrix3x3_make_rotation!(
        f64,
        make_rotation_z,
        Matrix3x3::<f64>::from_mat([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0],]),
        std::f64::consts::FRAC_PI_2
    );

    test_matrix3x3_make_rotation!(
        f32,
        make_rotation_z,
        Matrix3x3::<f32>::from_mat([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0],]),
        std::f32::consts::FRAC_PI_2
    );

    test_matrix3x3_make_rotation!(
        f64,
        make_rotation_z,
        Matrix3x3::<f64>::from_mat([[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0],]),
        std::f64::consts::PI
    );

    test_matrix3x3_make_rotation!(
        f32,
        make_rotation_z,
        Matrix3x3::<f32>::from_mat([[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0],]),
        std::f32::consts::PI
    );
}

#[test]
fn test_matrix3x3_make_rotation_x() {
    test_matrix3x3_make_rotation!(
        f64,
        make_rotation_x,
        Matrix3x3::<f64>::from_mat([[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0],]),
        std::f64::consts::FRAC_PI_2
    );

    test_matrix3x3_make_rotation!(
        f32,
        make_rotation_x,
        Matrix3x3::<f32>::from_mat([[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0],]),
        std::f32::consts::FRAC_PI_2
    );

    test_matrix3x3_make_rotation!(
        f64,
        make_rotation_x,
        Matrix3x3::<f64>::from_mat([[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0],]),
        std::f64::consts::PI
    );

    test_matrix3x3_make_rotation!(
        f32,
        make_rotation_x,
        Matrix3x3::<f32>::from_mat([[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0],]),
        std::f32::consts::PI
    );
}

#[test]
fn test_matrix3x3_make_rotation_y() {
    test_matrix3x3_make_rotation!(
        f64,
        make_rotation_y,
        Matrix3x3::<f64>::from_mat([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0],]),
        std::f64::consts::FRAC_PI_2
    );

    test_matrix3x3_make_rotation!(
        f32,
        make_rotation_y,
        Matrix3x3::<f32>::from_mat([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0],]),
        std::f32::consts::FRAC_PI_2
    );

    test_matrix3x3_make_rotation!(
        f64,
        make_rotation_y,
        Matrix3x3::<f64>::from_mat([[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0],]),
        std::f64::consts::PI
    );

    test_matrix3x3_make_rotation!(
        f32,
        make_rotation_y,
        Matrix3x3::<f32>::from_mat([[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0],]),
        std::f32::consts::PI
    );
}

#[test]
fn test_matrix3x3_make_rotation() {
    // Rotate over Z axis by 90 degrees (π/2 radians)
    let angle = std::f64::consts::FRAC_PI_2;
    let rot = Matrix3x3::<f64>::make_rotation(angle, &Vector3::new(0.0, 0.0, -1.0));
    let expected = Matrix3x3::<f64>::from_mat([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f64, rot, expected);

    // Rotate over Z axis by 90 degrees (π/2 radians)
    let angle = std::f32::consts::FRAC_PI_2;
    let rot = Matrix3x3::<f32>::make_rotation(angle, &Vector3::new(0.0, 0.0, -1.0));
    let expected = Matrix3x3::<f32>::from_mat([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f32, rot, expected);
}

#[test]
fn test_matrix3x3_make_scaling() {
    let scale = Matrix3x3::<f32>::make_scaling(2.0, 3.0, 1.0);
    let expected = Matrix3x3::<f32>::from_mat([[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq!(scale, expected);
}

#[test]
fn test_matrix3x3_make_reflection_f64() {
    // Reflect over yz-plane (x-axis)
    let plane_normal = Vector3::new(1.0, 0.0, 0.0);
    let reflect_x = Matrix3x3::<f64>::make_reflection(&plane_normal);
    let expected_x =
        Matrix3x3::<f64>::from_mat([[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f64, reflect_x, expected_x);

    // Reflect over xz-plane (y-axis)
    let plane_normal = Vector3::new(0.0, 1.0, 0.0);
    let reflect_y = Matrix3x3::<f64>::make_reflection(&plane_normal);
    let expected_y =
        Matrix3x3::<f64>::from_mat([[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f64, reflect_y, expected_y);

    // Reflect over the plane that cuts `x` and `y` axes at 45 degrees.
    let plane_normal = Vector3::<f64>::new(1.0, 1.0, 0.0).normalize();
    let reflect_xy = Matrix3x3::<f64>::make_reflection(&plane_normal);
    let expected_xy =
        Matrix3x3::<f64>::from_mat([[0.0, -1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f64, reflect_xy, expected_xy);
}

#[test]
fn test_matrix3x3_make_reflection_f32() {
    // Reflect over yz-plane (x-axis)
    let plane_normal = Vector3::new(1.0, 0.0, 0.0);
    let reflect_x = Matrix3x3::<f32>::make_reflection(&plane_normal);
    let expected_x =
        Matrix3x3::<f32>::from_mat([[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f32, reflect_x, expected_x);

    // Reflect over xz-plane (y-axis)
    let plane_normal = Vector3::new(0.0, 1.0, 0.0);
    let reflect_y = Matrix3x3::<f32>::make_reflection(&plane_normal);
    let expected_y =
        Matrix3x3::<f32>::from_mat([[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f32, reflect_y, expected_y);

    // Reflect over the plane that cuts `x` and `y` axes at 45 degrees.
    let plane_normal = Vector3::<f32>::new(1.0, 1.0, 0.0).normalize();
    let reflect_xy = Matrix3x3::<f32>::make_reflection(&plane_normal);
    let expected_xy =
        Matrix3x3::<f32>::from_mat([[0.0, -1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_eq_mat!(f32, reflect_xy, expected_xy);
}

#[test]
#[should_panic]
fn test_matrix3x3_make_reflection_zero_normal_f64() {
    let plane_normal = Vector3::new(0.0, 0.0, 0.0);
    let _reflect = Matrix3x3::<f64>::make_reflection(&plane_normal); // Noarmal cannot be origin
}

#[test]
#[should_panic]
fn test_matrix3x3_make_reflection_not_normalized_normal_f64() {
    let plane_normal = Vector3::new(2.0, 0.0, 0.0);
    let _reflect = Matrix3x3::<f64>::make_reflection(&plane_normal); // Normal must be normalized
}

#[test]
#[should_panic]
fn test_matrix3x3_make_reflection_zero_normal_f32() {
    let plane_normal = Vector3::new(0.0, 0.0, 0.0);
    let _reflect = Matrix3x3::<f32>::make_reflection(&plane_normal); // Normal cannot be origin
}

#[test]
#[should_panic]
fn test_matrix3x3_make_reflection_not_normalized_normal_f32() {
    let plane_normal = Vector3::new(2.0, 0.0, 0.0);
    let _reflect = Matrix3x3::<f32>::make_reflection(&plane_normal); // Normal must be normalized
}

#[test]
fn test_matrix3x3_make_skew_f64() {
    let direction = Vector3::new(0.0, 0.0, 1.0);
    let pivot = Vector3::new(1.0, 0.5, 0.0);
    let rad = std::f64::consts::FRAC_PI_4; // 45 degrees
    let skew = Matrix3x3::<f64>::make_skew(rad, &direction, &pivot);
    let expected = Matrix3x3::<f64>::from_mat([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.5, 1.0]]);
    assert_eq_mat!(f64, skew, expected);
}

#[test]
fn test_matrix3x3_make_skew_f32() {
    let direction = Vector3::new(0.0, 0.0, 1.0);
    let pivot = Vector3::new(1.0, 0.5, 0.0);
    let rad = std::f32::consts::FRAC_PI_4; // 45 degrees
    let skew = Matrix3x3::<f32>::make_skew(rad, &direction, &pivot);
    let expected = Matrix3x3::<f32>::from_mat([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.5, 1.0]]);
    assert_eq_mat!(f32, skew, expected);
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_zero_direction_f64() {
    let direction = Vector3::new(0.0, 0.0, 0.0);
    let pivot = Vector3::new(1.0, 0.5, 0.0);
    let rad = std::f64::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f64>::make_skew(rad, &direction, &pivot); // Direction must be normalized
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_zero_pivot_f64() {
    let direction = Vector3::new(0.0, 0.0, 1.0);
    let pivot = Vector3::new(0.0, 0.0, 0.0);
    let rad = std::f64::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f64>::make_skew(rad, &direction, &pivot); // Pivot at origin is not allowed
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_direction_not_normalized_f64() {
    let direction = Vector3::new(0.0, 0.0, 10.0);
    let pivot = Vector3::new(1.0, 0.5, 0.0);
    let rad = std::f64::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f64>::make_skew(rad, &direction, &pivot); // Direction must be normalized
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_pivot_not_perpendicular_f64() {
    let direction = Vector3::new(0.0, 0.0, 1.0);
    let pivot = Vector3::new(1.0, 0.5, 1.0);
    let rad = std::f64::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f64>::make_skew(rad, &direction, &pivot); // Pivot must be perpendicular to direction
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_zero_direction_f32() {
    let direction = Vector3::new(0.0, 0.0, 0.0);
    let pivot = Vector3::new(1.0, 0.5, 0.0);
    let rad = std::f32::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f32>::make_skew(rad, &direction, &pivot);
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_zero_pivot_f32() {
    let direction = Vector3::new(0.0, 0.0, 1.0);
    let pivot = Vector3::new(0.0, 0.0, 0.0);
    let rad = std::f32::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f32>::make_skew(rad, &direction, &pivot); // Pivot at origin is not allowed
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_direction_not_normalized_f32() {
    let direction = Vector3::new(0.0, 0.0, 10.0);
    let pivot = Vector3::new(1.0, 0.5, 0.0);
    let rad = std::f32::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f32>::make_skew(rad, &direction, &pivot); // Direction must be normalized
}

#[test]
#[should_panic]
fn test_matrix3x3_make_skew_pivot_not_perpendicular_f32() {
    let direction = Vector3::new(0.0, 0.0, 1.0);
    let pivot = Vector3::new(1.0, 0.5, 1.0);
    let rad = std::f32::consts::FRAC_PI_4; // 45 degrees
    let _skew = Matrix3x3::<f32>::make_skew(rad, &direction, &pivot); // Pivot must be perpendicular to direction
}
