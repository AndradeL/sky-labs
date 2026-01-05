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

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use super::{SignedNumber, Vector3, Vector4};

/// A 4x4 matrix represented as an array of four `Vector4<T>` as rows.
/// It supports addition, subtraction, multiplication by a scalar,
/// multiplication by another matrix, and multiplication by a vector.
/// It also provides indexing for accessing individual rows, so you can use `matrix[i]` to access the i-th row.
/// And `matrix[i][j]` to access the j-th element of the i-th row.
/// It is generic over any type `T` that implements the `SignedNumber` trait.
/// The matrix is stored in row-major order.
/// The transform matrices are designed for working with 3-dimensional coordinate systems
/// with quaternion support, and follow the right-handed coordinate system convention.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Matrix4x4<T: SignedNumber> {
    mat: [Vector4<T>; 4],
}

impl<T: SignedNumber> Add for Matrix4x4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            mat: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
            ],
        }
    }
}

impl<T: SignedNumber> AddAssign for Matrix4x4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}

impl<T: SignedNumber> Sub for Matrix4x4<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            mat: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
            ],
        }
    }
}

impl<T: SignedNumber> SubAssign for Matrix4x4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
        self[3] -= rhs[3];
    }
}

impl<T: SignedNumber> Mul<T> for Matrix4x4<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            mat: [self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs],
        }
    }
}

impl<T: SignedNumber> MulAssign<T> for Matrix4x4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
        self[3] *= rhs;
    }
}

impl<T: SignedNumber> Mul<Matrix4x4<T>> for Matrix4x4<T> {
    type Output = Self;

    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        Self {
            mat: [
                Vector4 {
                    x: self[0][0] * rhs[0][0]
                        + self[0][1] * rhs[1][0]
                        + self[0][2] * rhs[2][0]
                        + self[0][3] * rhs[3][0],
                    y: self[0][0] * rhs[0][1]
                        + self[0][1] * rhs[1][1]
                        + self[0][2] * rhs[2][1]
                        + self[0][3] * rhs[3][1],
                    z: self[0][0] * rhs[0][2]
                        + self[0][1] * rhs[1][2]
                        + self[0][2] * rhs[2][2]
                        + self[0][3] * rhs[3][2],
                    w: self[0][0] * rhs[0][3]
                        + self[0][1] * rhs[1][3]
                        + self[0][2] * rhs[2][3]
                        + self[0][3] * rhs[3][3],
                },
                Vector4 {
                    x: self[1][0] * rhs[0][0]
                        + self[1][1] * rhs[1][0]
                        + self[1][2] * rhs[2][0]
                        + self[1][3] * rhs[3][0],
                    y: self[1][0] * rhs[0][1]
                        + self[1][1] * rhs[1][1]
                        + self[1][2] * rhs[2][1]
                        + self[1][3] * rhs[3][1],
                    z: self[1][0] * rhs[0][2]
                        + self[1][1] * rhs[1][2]
                        + self[1][2] * rhs[2][2]
                        + self[1][3] * rhs[3][2],
                    w: self[1][0] * rhs[0][3]
                        + self[1][1] * rhs[1][3]
                        + self[1][2] * rhs[2][3]
                        + self[1][3] * rhs[3][3],
                },
                Vector4 {
                    x: self[2][0] * rhs[0][0]
                        + self[2][1] * rhs[1][0]
                        + self[2][2] * rhs[2][0]
                        + self[2][3] * rhs[3][0],
                    y: self[2][0] * rhs[0][1]
                        + self[2][1] * rhs[1][1]
                        + self[2][2] * rhs[2][1]
                        + self[2][3] * rhs[3][1],
                    z: self[2][0] * rhs[0][2]
                        + self[2][1] * rhs[1][2]
                        + self[2][2] * rhs[2][2]
                        + self[2][3] * rhs[3][2],
                    w: self[2][0] * rhs[0][3]
                        + self[2][1] * rhs[1][3]
                        + self[2][2] * rhs[2][3]
                        + self[2][3] * rhs[3][3],
                },
                Vector4 {
                    x: self[3][0] * rhs[0][0]
                        + self[3][1] * rhs[1][0]
                        + self[3][2] * rhs[2][0]
                        + self[3][3] * rhs[3][0],
                    y: self[3][0] * rhs[0][1]
                        + self[3][1] * rhs[1][1]
                        + self[3][2] * rhs[2][1]
                        + self[3][3] * rhs[3][1],
                    z: self[3][0] * rhs[0][2]
                        + self[3][1] * rhs[1][2]
                        + self[3][2] * rhs[2][2]
                        + self[3][3] * rhs[3][2],
                    w: self[3][0] * rhs[0][3]
                        + self[3][1] * rhs[1][3]
                        + self[3][2] * rhs[2][3]
                        + self[3][3] * rhs[3][3],
                },
            ],
        }
    }
}

impl<T: SignedNumber> MulAssign<Matrix4x4<T>> for Matrix4x4<T> {
    fn mul_assign(&mut self, rhs: Matrix4x4<T>) {
        *self = *self * rhs;
    }
}

/// Right-multiplication of a matrix by a vector.
impl<T: SignedNumber> Mul<Vector4<T>> for Matrix4x4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Vector4 {
            x: self[0].dot(&rhs),
            y: self[1].dot(&rhs),
            z: self[2].dot(&rhs),
            w: self[3].dot(&rhs),
        }
    }
}

/// Left-multiplication of a vector by a matrix.
impl<T: SignedNumber> Mul<Matrix4x4<T>> for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        Vector4 {
            x: self.x * rhs[0][0] + self.y * rhs[1][0] + self.z * rhs[2][0] + self.w * rhs[3][0],
            y: self.x * rhs[0][1] + self.y * rhs[1][1] + self.z * rhs[2][1] + self.w * rhs[3][1],
            z: self.x * rhs[0][2] + self.y * rhs[1][2] + self.z * rhs[2][2] + self.w * rhs[3][2],
            w: self.x * rhs[0][3] + self.y * rhs[1][3] + self.z * rhs[2][3] + self.w * rhs[3][3],
        }
    }
}

impl<T: SignedNumber> Index<usize> for Matrix4x4<T>
where
    T: SignedNumber,
{
    type Output = Vector4<T>;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);
        &self.mat[index]
    }
}

impl<T: SignedNumber> IndexMut<usize> for Matrix4x4<T>
where
    T: SignedNumber,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);
        &mut self.mat[index]
    }
}

impl<T: SignedNumber> Index<(usize, usize)> for Matrix4x4<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(index.0 < 4 && index.1 < 4);
        &self.mat[index.0][index.1]
    }
}

impl<T: SignedNumber> IndexMut<(usize, usize)> for Matrix4x4<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        debug_assert!(index.0 < 4 && index.1 < 4);
        &mut self.mat[index.0][index.1]
    }
}

impl<T: SignedNumber> Matrix4x4<T> {
    /// Creates a new `Matrix4x4` with the given rows.
    pub fn new(rows: [Vector4<T>; 4]) -> Self {
        Self { mat: rows }
    }

    /// Creates a new `Matrix4x4` with all elements initialized to zero.
    pub fn zero() -> Self {
        Self {
            mat: [
                Vector4::zero(),
                Vector4::zero(),
                Vector4::zero(),
                Vector4::zero(),
            ],
        }
    }

    /// Creates a new `Matrix4x4` with all elements initialized to one.
    pub fn one() -> Self {
        Self {
            mat: [
                Vector4::one(),
                Vector4::one(),
                Vector4::one(),
                Vector4::one(),
            ],
        }
    }

    /// Creates a new `Matrix4x4` that represents the identity matrix.
    /// The identity matrix has ones on the diagonal and zeros elsewhere.
    pub fn identity() -> Self {
        Self {
            mat: [
                Vector4 {
                    x: T::one(),
                    y: T::zero(),
                    z: T::zero(),
                    w: T::zero(),
                },
                Vector4 {
                    x: T::zero(),
                    y: T::one(),
                    z: T::zero(),
                    w: T::zero(),
                },
                Vector4 {
                    x: T::zero(),
                    y: T::zero(),
                    z: T::one(),
                    w: T::zero(),
                },
                Vector4 {
                    x: T::zero(),
                    y: T::zero(),
                    z: T::zero(),
                    w: T::one(),
                },
            ],
        }
    }

    /// Returns the transpose of the matrix.
    /// The transpose is obtained by swapping rows and columns.
    /// For each element `mat[i][j]`, the transpose will have `mat[j][i]`.
    pub fn transpose(&self) -> Self {
        Self {
            mat: [
                Vector4::new(self[0][0], self[1][0], self[2][0], self[3][0]),
                Vector4::new(self[0][1], self[1][1], self[2][1], self[3][1]),
                Vector4::new(self[0][2], self[1][2], self[2][2], self[3][2]),
                Vector4::new(self[0][3], self[1][3], self[2][3], self[3][3]),
            ],
        }
    }

    /// Returns the determinant of the matrix.
    pub fn determinant(&self) -> T {
        self[0][0]
            * (self[1][1] * (self[2][2] * self[3][3] - self[2][3] * self[3][2])
                - self[1][2] * (self[2][1] * self[3][3] - self[2][3] * self[3][1])
                + self[1][3] * (self[2][1] * self[3][2] - self[2][2] * self[3][1]))
            - self[0][1]
                * (self[1][0] * (self[2][2] * self[3][3] - self[2][3] * self[3][2])
                    - self[1][2] * (self[2][0] * self[3][3] - self[2][3] * self[3][0])
                    + self[1][3] * (self[2][0] * self[3][2] - self[2][2] * self[3][0]))
            + self[0][2]
                * (self[1][0] * (self[2][1] * self[3][3] - self[2][3] * self[3][1])
                    - self[1][1] * (self[2][0] * self[3][3] - self[2][3] * self[3][0])
                    + self[1][3] * (self[2][0] * self[3][1] - self[2][1] * self[3][0]))
            - self[0][3]
                * (self[1][0] * (self[2][1] * self[3][2] - self[2][2] * self[3][1])
                    - self[1][1] * (self[2][0] * self[3][2] - self[2][2] * self[3][0])
                    + self[1][2] * (self[2][0] * self[3][1] - self[2][1] * self[3][0]))
    }

    pub fn inverse(&self) -> Option<Self> {
        let col0 = Vector3::<T>::new(self[0][0], self[1][0], self[2][0]);
        let col1 = Vector3::<T>::new(self[0][1], self[1][1], self[2][1]);
        let col2 = Vector3::<T>::new(self[0][2], self[1][2], self[2][2]);
        let col3 = Vector3::<T>::new(self[0][3], self[1][3], self[2][3]);

        let x = self[3][0];
        let y = self[3][1];
        let z = self[3][2];
        let w = self[3][3];

        let s = col0.cross(&col1);
        let t = col2.cross(&col3);
        let u = col0 * y - col1 * x;
        let v = col2 * w - col3 * z;

        let determinant = s.dot(&v) + t.dot(&u);
        if determinant == T::zero() {
            return None; // Matrix is singular, no inverse exists
        }

        let inv_det = T::one() / determinant;
        let s = s * inv_det;
        let t = t * inv_det;
        let u = u * inv_det;
        let v = v * inv_det;

        let r0 = col1.cross(&v) + t * y;
        let r1 = v.cross(&col0) - t * x;
        let r2 = col3.cross(&u) + s * w;
        let r3 = u.cross(&col2) - s * z;

        Some(Self {
            mat: [
                Vector4::from_vector3(&r0, -col1.dot(&t)),
                Vector4::from_vector3(&r1, col0.dot(&t)),
                Vector4::from_vector3(&r2, -col3.dot(&s)),
                Vector4::from_vector3(&r3, col2.dot(&s)),
            ],
        })
    }

    /// Returns the rows of the matrix as an array of `Vector4<T>`.
    pub fn rows(&self) -> &[Vector4<T>; 4] {
        &self.mat
    }

    /// Returns mutable access to the rows of the matrix as an array of `Vector4<T>`.
    pub fn rows_mut(&mut self) -> &mut [Vector4<T>; 4] {
        &mut self.mat
    }

    /// Creates a `Matrix4x4` from a 2D array.
    pub const fn from_mat(mat: [[T; 4]; 4]) -> Self {
        Self {
            mat: [
                Vector4::from_array(mat[0]),
                Vector4::from_array(mat[1]),
                Vector4::from_array(mat[2]),
                Vector4::from_array(mat[3]),
            ],
        }
    }

    /// Converts the `Matrix4x4` to a 2D array.
    pub const fn to_mat(&self) -> [[T; 4]; 4] {
        [
            self.mat[0].to_array(),
            self.mat[1].to_array(),
            self.mat[2].to_array(),
            self.mat[3].to_array(),
        ]
    }

    /// Creates a `Matrix4x4` from a flat array of 16 elements.
    /// The elements are arranged in row-major order.
    pub const fn from_array(arr: [T; 16]) -> Self {
        Self {
            mat: [
                Vector4::from_array([arr[0], arr[1], arr[2], arr[3]]),
                Vector4::from_array([arr[4], arr[5], arr[6], arr[7]]),
                Vector4::from_array([arr[8], arr[9], arr[10], arr[11]]),
                Vector4::from_array([arr[12], arr[13], arr[14], arr[15]]),
            ],
        }
    }

    /// Converts the `Matrix4x4` to a flat array of 16 elements.
    /// The elements are arranged in row-major order.
    pub const fn to_array(&self) -> [T; 16] {
        [
            self.mat[0].x,
            self.mat[0].y,
            self.mat[0].z,
            self.mat[0].w,
            self.mat[1].x,
            self.mat[1].y,
            self.mat[1].z,
            self.mat[1].w,
            self.mat[2].x,
            self.mat[2].y,
            self.mat[2].z,
            self.mat[2].w,
            self.mat[3].x,
            self.mat[3].y,
            self.mat[3].z,
            self.mat[3].w,
        ]
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        debug_assert!(slice.len() < 16, "Matrix4x4 must have at least 16 elements");
        Self {
            mat: [
                Vector4::new(slice[0], slice[1], slice[2], slice[3]),
                Vector4::new(slice[4], slice[5], slice[6], slice[7]),
                Vector4::new(slice[8], slice[9], slice[10], slice[11]),
                Vector4::new(slice[12], slice[13], slice[14], slice[15]),
            ],
        }
    }

    pub fn as_slice(&self) -> &[T; 16] {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T; 16] {
        unsafe { std::mem::transmute(self) }
    }

    pub unsafe fn as_ptr(&self) -> *const T {
        self[0].as_ptr()
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        self[0].as_mut_ptr()
    }
}

impl Matrix4x4<f32> {
    /// Creates a translation matrix that translates points by the specified amounts along each axis.
    pub fn make_translation(tx: f32, ty: f32, tz: f32) -> Self {
        Self::from_mat([
            [1.0, 0.0, 0.0, tx],
            [0.0, 1.0, 0.0, ty],
            [0.0, 0.0, 1.0, tz],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a transform matrix to rotate around the X-axis.
    /// This matrix rotates points in the YZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_x(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self::from_mat([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a transform matrix to rotate around the Y-axis.
    /// This matrix rotates points in the XZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_y(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self::from_mat([
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a transform matrix to rotate around the Z-axis.
    /// This matrix rotates points in the XY plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_z(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self::from_mat([
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a rotation matrix around an arbitrary axis.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation(rad: f32, axis: &Vector3<f32>) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let one_minus_cos = 1.0 - cos;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Self::from_mat([
            [
                cos + x * x * one_minus_cos,
                y * x * one_minus_cos + z * sin,
                z * x * one_minus_cos - y * sin,
                0.0,
            ],
            [
                x * y * one_minus_cos - z * sin,
                cos + y * y * one_minus_cos,
                z * y * one_minus_cos + x * sin,
                0.0,
            ],
            [
                x * z * one_minus_cos + y * sin,
                y * z * one_minus_cos - x * sin,
                cos + z * z * one_minus_cos,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a scaling matrix that scales points by the specified factors along each axis.
    pub fn make_scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Self::from_mat([
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a scaling matrix that scales points along the specified axis by the given factor.
    /// Assumes the axis is normalized.
    pub fn make_scaling_axis(axis: &Vector3<f32>, factor: f32) -> Self {
        debug_assert!(axis.is_normalized(), "Axis must be normalized");
        let x = axis.x * factor;
        let y = axis.y * factor;
        let z = axis.z * factor;
        Self::from_mat([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a reflection matrix that reflects points through the specified plane.
    /// The plane is defined by its normal vector.
    /// Assumes the normal vector is normalized.
    pub fn make_reflection(normal: &Vector3<f32>) -> Self {
        debug_assert!(normal.is_normalized(), "Normal vector must be normalized");
        let x = normal.x * -2.0;
        let y = normal.y * -2.0;
        let z = normal.z * -2.0;
        Self::from_mat([
            [1.0 + x * normal.x, x * normal.y, x * normal.z, 0.0],
            [y * normal.x, 1.0 + y * normal.y, y * normal.z, 0.0],
            [z * normal.x, z * normal.y, 1.0 + z * normal.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a skew transformation matrix that skews points by `rad` along
    /// the `direction` in regards to the `pivot` axis, which is used to
    /// measure the distance to determine how far to skew.
    /// It assumes the `direction` vector is normalized and
    /// the `pivot` is non-zero and perpendicular to the `direction` vector.
    pub fn make_skew(rad: f32, direction: &Vector3<f32>, pivot: &Vector3<f32>) -> Self {
        debug_assert!(direction.is_normalized(), "`direction` must be normalized");
        debug_assert!(pivot.magnitude() > 0.0, "`pivot` must not be origin");
        debug_assert!(
            pivot.dot(&direction) == 0.0,
            "`pivot` must be perpendicular to `direction`"
        );

        let tan = rad.tan();
        let x = direction.x * tan;
        let y = direction.y * tan;
        let z = direction.z * tan;

        Self::from_mat([
            [x * pivot.x + 1.0, x * pivot.y, x * pivot.z, 0.0],
            [y * pivot.x, y * pivot.y + 1.0, y * pivot.z, 0.0],
            [z * pivot.x, z * pivot.y, z * pivot.z + 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Matrix4x4<f64> {
    /// Creates a translation matrix that translates points by the specified amounts along each axis.
    pub fn make_translation(tx: f64, ty: f64, tz: f64) -> Self {
        Self::from_mat([
            [1.0, 0.0, 0.0, tx],
            [0.0, 1.0, 0.0, ty],
            [0.0, 0.0, 1.0, tz],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a transform matrix to rotate around the X-axis.
    /// This matrix rotates points in the YZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_x(rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self::from_mat([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a transform matrix to rotate around the Y-axis.
    /// This matrix rotates points in the XZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_y(rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self::from_mat([
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a transform matrix to rotate around the Z-axis.
    /// This matrix rotates points in the XY plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_z(rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self::from_mat([
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a transform matrix to rotate around an arbitrary axis.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation(rad: f64, axis: &Vector3<f64>) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let one_minus_cos = 1.0 - cos;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Self::from_mat([
            [
                cos + x * x * one_minus_cos,
                y * x * one_minus_cos + z * sin,
                z * x * one_minus_cos + y * sin,
                0.0,
            ],
            [
                x * y * one_minus_cos - z * sin,
                cos + y * y * one_minus_cos,
                z * y * one_minus_cos + x * sin,
                0.0,
            ],
            [
                x * z * one_minus_cos + y * sin,
                y * z * one_minus_cos - x * sin,
                cos + z * z * one_minus_cos,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a scaling matrix that scales points by the specified factors along each axis.
    pub fn make_scaling(sx: f64, sy: f64, sz: f64) -> Self {
        Self::from_mat([
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a scaling matrix that scales points along the specified axis by the given factor.
    /// Assumes the axis is normalized.
    pub fn make_scaling_axis(axis: &Vector3<f64>, factor: f64) -> Self {
        debug_assert!(axis.is_normalized(), "`axis` must be a normalized");
        let x = axis.x * factor;
        let y = axis.y * factor;
        let z = axis.z * factor;
        Self::from_mat([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a reflection matrix that reflects points through the specified plane.
    /// The plane is defined by its normal vector.
    /// Assumes the normal vector is normalized.
    pub fn make_reflection(normal: &Vector3<f64>) -> Self {
        debug_assert!(normal.is_normalized(), "`normal` must be normalized");
        let x = normal.x * -2.0;
        let y = normal.y * -2.0;
        let z = normal.z * -2.0;
        Self::from_mat([
            [1.0 + x * normal.x, x * normal.y, x * normal.z, 0.0],
            [y * normal.x, 1.0 + y * normal.y, y * normal.z, 0.0],
            [z * normal.x, z * normal.y, 1.0 + z * normal.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a skew transformation matrix that skews points by `rad` along
    /// the `direction` in regards to the `pivot` axis, which is used to
    /// measure the distance to determine how far to skew.
    /// It assumes the `direction` vector is normalized and
    /// the `pivot` is non-zero and perpendicular to the `direction` vector.
    pub fn make_skew(rad: f64, direction: &Vector3<f64>, pivot: &Vector3<f64>) -> Self {
        debug_assert!(direction.is_normalized(), "`direction` must be normalized");
        debug_assert!(pivot.magnitude() > 0.0, "`pivot` must not be origin");
        debug_assert!(
            pivot.dot(&direction) == 0.0,
            "`pivot` must be perpendicular to `direction`"
        );

        let tan = rad.tan();
        let x = direction.x * tan;
        let y = direction.y * tan;
        let z = direction.z * tan;

        Self::from_mat([
            [x * pivot.x + 1.0, x * pivot.y, x * pivot.z, 0.0],
            [y * pivot.x, y * pivot.y + 1.0, y * pivot.z, 0.0],
            [z * pivot.x, z * pivot.y, z * pivot.z + 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
