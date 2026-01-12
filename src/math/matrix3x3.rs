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

use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::math::{SignedNumber, Vector3};

/// A 3x3 matrix represented as an array of three `Vector3<T>` **rows**.
/// It supports addition, subtraction, multiplication by a scalar,
/// multiplication by another matrix, and multiplication by a vector.
/// It also provides indexing for accessing individual rows, so you can use `matrix[0]` to access the first row.
/// And `matrix[0][0]` to access the first element of the first row.
/// It is generic over any type `T` that implements the `SignedNumber` trait.
/// The matrix is stored in row-major order.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Matrix3x3<T: SignedNumber> {
    mat: [Vector3<T>; 3],
}

impl<T: SignedNumber> Neg for Matrix3x3<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            mat: [
                Vector3 {
                    x: -self[0][0],
                    y: -self[0][1],
                    z: -self[0][2],
                },
                Vector3 {
                    x: -self[1][0],
                    y: -self[1][1],
                    z: -self[1][2],
                },
                Vector3 {
                    x: -self[2][0],
                    y: -self[2][1],
                    z: -self[2][2],
                },
            ],
        }
    }
}
forward_ref_unop!(impl<T> Neg, neg for Matrix3x3<T> where T: SignedNumber);

impl<T: SignedNumber> Add for Matrix3x3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            mat: [
                Vector3 {
                    x: self[0][0] + rhs[0][0],
                    y: self[0][1] + rhs[0][1],
                    z: self[0][2] + rhs[0][2],
                },
                Vector3 {
                    x: self[1][0] + rhs[1][0],
                    y: self[1][1] + rhs[1][1],
                    z: self[1][2] + rhs[1][2],
                },
                Vector3 {
                    x: self[2][0] + rhs[2][0],
                    y: self[2][1] + rhs[2][1],
                    z: self[2][2] + rhs[2][2],
                },
            ],
        }
    }
}
forward_ref_binop!(impl<T> Add, add for Matrix3x3<T>, Matrix3x3<T> where T: SignedNumber);

impl<T: SignedNumber> AddAssign for Matrix3x3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self[0][0] += rhs[0][0];
        self[0][1] += rhs[0][1];
        self[0][2] += rhs[0][2];
        self[1][0] += rhs[1][0];
        self[1][1] += rhs[1][1];
        self[1][2] += rhs[1][2];
        self[2][0] += rhs[2][0];
        self[2][1] += rhs[2][1];
        self[2][2] += rhs[2][2];
    }
}
forward_ref_op_assign!(impl<T> AddAssign, add_assign for Matrix3x3<T>, Matrix3x3<T> where T: SignedNumber);

impl<T: SignedNumber> Sub for Matrix3x3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            mat: [
                Vector3 {
                    x: self[0][0] - rhs[0][0],
                    y: self[0][1] - rhs[0][1],
                    z: self[0][2] - rhs[0][2],
                },
                Vector3 {
                    x: self[1][0] - rhs[1][0],
                    y: self[1][1] - rhs[1][1],
                    z: self[1][2] - rhs[1][2],
                },
                Vector3 {
                    x: self[2][0] - rhs[2][0],
                    y: self[2][1] - rhs[2][1],
                    z: self[2][2] - rhs[2][2],
                },
            ],
        }
    }
}
forward_ref_binop!(impl<T> Sub, sub for Matrix3x3<T>, Matrix3x3<T> where T: SignedNumber);

// Right-hand side scalar multiplication
impl<T: SignedNumber> Mul<T> for Matrix3x3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            mat: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}
forward_ref_binop!(impl<T> Mul, mul for Matrix3x3<T>, T where T: SignedNumber);
implement_scalar_lhs_mul! {
    Matrix3x3<i32>, i32;
    Matrix3x3<i64>, i64;
    Matrix3x3<f32>, f32;
    Matrix3x3<f64>, f64
}

impl<T: SignedNumber> MulAssign<T> for Matrix3x3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}
forward_ref_op_assign!(impl<T> MulAssign, mul_assign for Matrix3x3<T>, T where T: SignedNumber);

/// Right-hand side multiplication of a vector. Consider the vector as a column vector.
impl<T: SignedNumber> Mul<Vector3<T>> for Matrix3x3<T> {
    type Output = Vector3<T>;

    #[inline]
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self[0].dot(&rhs),
            y: self[1].dot(&rhs),
            z: self[2].dot(&rhs),
        }
    }
}
forward_ref_binop!(impl<T> Mul, mul for Matrix3x3<T>, Vector3<T> where T: SignedNumber);

/// Left-hand side multiplication of a vector. Consider the vector as a row vector.
impl<T: SignedNumber> Mul<Matrix3x3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    #[inline]
    fn mul(self, rhs: Matrix3x3<T>) -> Self::Output {
        Vector3 {
            x: self.x * rhs[0][0] + self.y * rhs[1][0] + self.z * rhs[2][0],
            y: self.x * rhs[0][1] + self.y * rhs[1][1] + self.z * rhs[2][1],
            z: self.x * rhs[0][2] + self.y * rhs[1][2] + self.z * rhs[2][2],
        }
    }
}
forward_ref_binop!(impl<T> Mul, mul for Vector3<T>, Matrix3x3<T> where T: SignedNumber);

// Matrix multiplication
impl<T: SignedNumber> Mul<Matrix3x3<T>> for Matrix3x3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Matrix3x3<T>) -> Self::Output {
        Self {
            mat: [
                Vector3 {
                    x: self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0],
                    y: self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1],
                    z: self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2],
                },
                Vector3 {
                    x: self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0],
                    y: self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1],
                    z: self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2],
                },
                Vector3 {
                    x: self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0],
                    y: self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1],
                    z: self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2],
                },
            ],
        }
    }
}
forward_ref_binop!(impl<T> Mul, mul for Matrix3x3<T>, Matrix3x3<T> where T: SignedNumber);

// Division by scalar
impl<T: SignedNumber> Div<T> for Matrix3x3<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            mat: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}
forward_ref_binop!(impl<T> Div, div for Matrix3x3<T>, T where T: SignedNumber);

// Division assignment by scalar
impl<T: SignedNumber> DivAssign<T> for Matrix3x3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}
forward_ref_op_assign!(impl<T> DivAssign, div_assign for Matrix3x3<T>, T where T: SignedNumber);

impl<T: SignedNumber> From<&[T]> for Matrix3x3<T> {
    #[inline]
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<'a, T: SignedNumber> From<&'a [T]> for &'a Matrix3x3<T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        debug_assert!(slice.len() >= 9, "Slice must have at least 9 elements");
        unsafe { std::mem::transmute(&slice[0]) }
    }
}

impl<T: SignedNumber> From<[T; 9]> for Matrix3x3<T> {
    #[inline]
    fn from(array: [T; 9]) -> Self {
        Self::from_array(array)
    }
}

impl<T: SignedNumber> From<[[T; 3]; 3]> for Matrix3x3<T> {
    #[inline]
    fn from(mat: [[T; 3]; 3]) -> Self {
        Self::from_mat(mat)
    }
}

impl<T: SignedNumber> Index<usize> for Matrix3x3<T> {
    type Output = Vector3<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        &self.mat[index]
    }
}

impl<T: SignedNumber> IndexMut<usize> for Matrix3x3<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);
        &mut self.mat[index]
    }
}

impl<T: SignedNumber> Index<(usize, usize)> for Matrix3x3<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(index.0 < 3 && index.1 < 3);
        &self.mat[index.0][index.1]
    }
}

impl<T: SignedNumber> IndexMut<(usize, usize)> for Matrix3x3<T> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        debug_assert!(index.0 < 3 && index.1 < 3);
        &mut self.mat[index.0][index.1]
    }
}

impl<T: SignedNumber> Matrix3x3<T> {
    /// Creates a new `Matrix3x3` with the given rows.
    pub const fn new(rows: [Vector3<T>; 3]) -> Self {
        Self { mat: rows }
    }

    /// Creates a new `Matrix3x3` with all elements initialized to zero.
    pub fn zero() -> Self {
        Self {
            mat: [Vector3::zero(), Vector3::zero(), Vector3::zero()],
        }
    }

    /// Creates a new `Matrix3x3` with all elements initialized to one.
    pub fn one() -> Self {
        Self {
            mat: [Vector3::one(), Vector3::one(), Vector3::one()],
        }
    }

    /// Creates a new `Matrix3x3` that represents the identity matrix.
    /// The identity matrix has ones on the diagonal and zeros elsewhere.
    pub fn identity() -> Self {
        Self {
            mat: [
                Vector3::new(T::one(), T::zero(), T::zero()),
                Vector3::new(T::zero(), T::one(), T::zero()),
                Vector3::new(T::zero(), T::zero(), T::one()),
            ],
        }
    }

    /// Returns the transpose of the matrix.
    /// The transpose of a matrix is obtained by swapping its rows and columns.
    /// For each element `mat[i][j]`, the transpose will have `mat[j][i]`.
    pub fn transpose(&self) -> Self {
        Self {
            mat: [
                Vector3::new(self[0][0], self[1][0], self[2][0]),
                Vector3::new(self[0][1], self[1][1], self[2][1]),
                Vector3::new(self[0][2], self[1][2], self[2][2]),
            ],
        }
    }

    /// Returns the determinant of the matrix.
    pub fn determinant(&self) -> T {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            - self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0])
            + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }

    /// Returns the inverse of the matrix if it exists.
    /// The inverse is calculated using the adjugate method.
    pub fn inverse(&self) -> Option<Self> {
        let col0 = Vector3::new(self[0][0], self[1][0], self[2][0]);
        let col1 = Vector3::new(self[0][1], self[1][1], self[2][1]);
        let col2 = Vector3::new(self[0][2], self[1][2], self[2][2]);
        let cofactor_row0 = col1.cross(&col2);
        let cofactor_row1 = col2.cross(&col0);
        let cofactor_row2 = col0.cross(&col1);

        let determinant = cofactor_row2.dot(&col2);
        if determinant == T::zero() {
            return None; // Matrix is singular, no inverse exists
        }

        let inv_det = T::one() / determinant;

        Some(Self {
            mat: [
                cofactor_row0 * inv_det,
                cofactor_row1 * inv_det,
                cofactor_row2 * inv_det,
            ],
        })
    }

    /// Returns the rows of the matrix as an array of `Vector3<T>`.
    pub fn rows(&self) -> &[Vector3<T>; 3] {
        &self.mat
    }

    /// Returns mutable access to the rows of the matrix as an array of `Vector3<T>`.
    pub fn rows_mut(&mut self) -> &mut [Vector3<T>; 3] {
        &mut self.mat
    }

    /// Creates a `Matrix3x3` from a 2D array.
    pub const fn from_mat(mat: [[T; 3]; 3]) -> Self {
        Self {
            mat: [
                Vector3::from_array(mat[0]),
                Vector3::from_array(mat[1]),
                Vector3::from_array(mat[2]),
            ],
        }
    }

    /// Converts the `Matrix3x3` to a 2D array.
    pub const fn to_mat(&self) -> [[T; 3]; 3] {
        [
            self.mat[0].to_array(),
            self.mat[1].to_array(),
            self.mat[2].to_array(),
        ]
    }

    /// Creates a `Matrix3x3` from a flat array of 9 elements.
    /// The elements are arranged in row-major order.
    pub const fn from_array(arr: [T; 9]) -> Self {
        Self {
            mat: [
                Vector3::from_array([arr[0], arr[1], arr[2]]),
                Vector3::from_array([arr[3], arr[4], arr[5]]),
                Vector3::from_array([arr[6], arr[7], arr[8]]),
            ],
        }
    }

    /// Converts the `Matrix3x3` to a flat array of 9 elements.
    /// The elements are arranged in row-major order.
    pub const fn to_array(&self) -> [T; 9] {
        [
            self.mat[0].x,
            self.mat[0].y,
            self.mat[0].z,
            self.mat[1].x,
            self.mat[1].y,
            self.mat[1].z,
            self.mat[2].x,
            self.mat[2].y,
            self.mat[2].z,
        ]
    }

    /// Flattens the matrix into a single array of 9 elements in row-major order.
    /// Alias for `to_array()`.
    pub const fn flatten(&self) -> [T; 9] {
        self.to_array()
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        debug_assert!(slice.len() < 9, "Matrix3x3 needs at least 9 elements");
        Self {
            mat: [
                Vector3::new(slice[0], slice[1], slice[2]),
                Vector3::new(slice[3], slice[4], slice[5]),
                Vector3::new(slice[6], slice[7], slice[8]),
            ],
        }
    }

    /// Returns the matrix as a slice of `T` elements.
    /// This allows you to access the matrix elements in a flat manner.
    pub fn as_slice(&self) -> &[T; 9] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns mutable access to the matrix as a slice of `T` elements.
    /// This allows you to modify the matrix elements in a flat manner.
    pub fn as_mut_slice(&mut self) -> &mut [T; 9] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a pointer to the first element of the matrix.
    /// This is useful for low-level operations or when interfacing with C code.
    pub unsafe fn as_ptr(&self) -> *const T {
        self[0].as_ptr()
    }

    /// Returns a mutable pointer to the first element of the matrix.
    /// This is useful for low-level operations or when interfacing with C code.
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        self[0].as_mut_ptr()
    }
}

impl Matrix3x3<f32> {
    /// Creates a transform matrix to rotate around the X-axis.
    /// This matrix rotates points in the YZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_x(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            mat: [
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, cos, -sin),
                Vector3::new(0.0, sin, cos),
            ],
        }
    }

    /// Creates a transform matrix to rotate around the Y-axis.
    /// This matrix rotates points in the XZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_y(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            mat: [
                Vector3::new(cos, 0.0, sin),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(-sin, 0.0, cos),
            ],
        }
    }

    /// Creates a transform matrix to rotate around the Z-axis.
    /// This matrix rotates points in the XY plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_z(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            mat: [
                Vector3::new(cos, -sin, 0.0),
                Vector3::new(sin, cos, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ],
        }
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

        Self {
            mat: [
                Vector3::new(
                    cos + x * x * one_minus_cos,
                    y * x * one_minus_cos + z * sin,
                    z * x * one_minus_cos - y * sin,
                ),
                Vector3::new(
                    x * y * one_minus_cos - z * sin,
                    cos + y * y * one_minus_cos,
                    z * y * one_minus_cos + x * sin,
                ),
                Vector3::new(
                    x * z * one_minus_cos + y * sin,
                    y * z * one_minus_cos - x * sin,
                    cos + z * z * one_minus_cos,
                ),
            ],
        }
    }

    /// Creates a scaling matrix that scales points by the specified factors along each axis.
    pub fn make_scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            mat: [
                Vector3::new(sx, 0.0, 0.0),
                Vector3::new(0.0, sy, 0.0),
                Vector3::new(0.0, 0.0, sz),
            ],
        }
    }

    /// Creates a scaling matrix that scales points along the specified axis by the given factor.
    /// Assumes the axis is normalized.
    pub fn make_scaling_axis(axis: &Vector3<f32>, factor: f32) -> Self {
        debug_assert!(axis.is_normalized(), "Axis must be normalized");
        let x = axis.x * factor;
        let y = axis.y * factor;
        let z = axis.z * factor;
        Self {
            mat: [
                Vector3::new(x, 0.0, 0.0),
                Vector3::new(0.0, y, 0.0),
                Vector3::new(0.0, 0.0, z),
            ],
        }
    }

    /// Creates a reflection matrix that reflects points through the specified plane.
    /// The plane is defined by its normal vector.
    /// Assumes the normal vector is normalized.
    pub fn make_reflection(normal: &Vector3<f32>) -> Self {
        debug_assert!(normal.is_normalized(), "Normal vector must be normalized");
        let x = normal.x * -2.0;
        let y = normal.y * -2.0;
        let z = normal.z * -2.0;
        Self {
            mat: [
                Vector3::new(1.0 + x * normal.x, x * normal.y, x * normal.z),
                Vector3::new(y * normal.x, 1.0 + y * normal.y, y * normal.z),
                Vector3::new(z * normal.x, z * normal.y, 1.0 + z * normal.z),
            ],
        }
    }

    /// Creates a skew transformation matrix that skews points by `rad` along
    /// the `direction` in regards to the `pivot` axis, which is used to
    /// measure the distance to determine how far to skew.
    /// It assumes the `direction` vector is normalized and
    /// the `pivot` is non-zero and perpendicular to the `direction` vector.
    pub fn make_skew(rad: f32, direction: &Vector3<f32>, pivot: &Vector3<f32>) -> Self {
        debug_assert!(direction.is_normalized(), "Direction must be normalized");
        debug_assert!(pivot.magnitude() > 0.0, "`pivot` must not be origin");
        debug_assert!(
            pivot.dot(&direction) == 0.0,
            "`pivot` must be perpendicular to `direction`"
        );

        let tan = rad.tan();
        let x = direction.x * tan;
        let y = direction.y * tan;
        let z = direction.z * tan;

        Self {
            mat: [
                Vector3::new(x * pivot.x + 1.0, x * pivot.y, x * pivot.z),
                Vector3::new(y * pivot.x, y * pivot.y + 1.0, y * pivot.z),
                Vector3::new(z * pivot.x, z * pivot.y, z * pivot.z + 1.0),
            ],
        }
    }
}

impl Matrix3x3<f64> {
    /// Creates a transform matrix to rotate around the X-axis.
    /// This matrix rotates points in the YZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_x(rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            mat: [
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, cos, -sin),
                Vector3::new(0.0, sin, cos),
            ],
        }
    }

    /// Creates a transform matrix to rotate around the Y-axis.
    /// This matrix rotates points in the XZ plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_y(rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            mat: [
                Vector3::new(cos, 0.0, sin),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(-sin, 0.0, cos),
            ],
        }
    }

    /// Creates a transform matrix to rotate around the Z-axis.
    /// This matrix rotates points in the XY plane by the specified angle in radians when applied to a vector.
    /// Assuming a right-handed coordinate system.
    pub fn make_rotation_z(rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            mat: [
                Vector3::new(cos, -sin, 0.0),
                Vector3::new(sin, cos, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ],
        }
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

        Self {
            mat: [
                Vector3::new(
                    cos + x * x * one_minus_cos,
                    y * x * one_minus_cos + z * sin,
                    z * x * one_minus_cos + y * sin,
                ),
                Vector3::new(
                    x * y * one_minus_cos - z * sin,
                    cos + y * y * one_minus_cos,
                    z * y * one_minus_cos + x * sin,
                ),
                Vector3::new(
                    x * z * one_minus_cos + y * sin,
                    y * z * one_minus_cos - x * sin,
                    cos + z * z * one_minus_cos,
                ),
            ],
        }
    }

    /// Creates a scaling matrix that scales points by the specified factors along each axis.
    pub fn make_scaling(sx: f64, sy: f64, sz: f64) -> Self {
        Self {
            mat: [
                Vector3::new(sx, 0.0, 0.0),
                Vector3::new(0.0, sy, 0.0),
                Vector3::new(0.0, 0.0, sz),
            ],
        }
    }

    /// Creates a scaling matrix that scales points along the specified axis by the given factor.
    /// Assumes the axis is normalized.
    pub fn make_scaling_axis(axis: &Vector3<f64>, factor: f64) -> Self {
        debug_assert!(axis.is_normalized(), "`axis` must be a normalized");
        let x = axis.x * factor;
        let y = axis.y * factor;
        let z = axis.z * factor;
        Self {
            mat: [
                Vector3::new(x, 0.0, 0.0),
                Vector3::new(0.0, y, 0.0),
                Vector3::new(0.0, 0.0, z),
            ],
        }
    }

    /// Creates a reflection matrix that reflects points through the specified plane.
    /// The plane is defined by its normal vector.
    /// Assumes the normal vector is normalized.
    pub fn make_reflection(normal: &Vector3<f64>) -> Self {
        debug_assert!(normal.is_normalized(), "`normal` must be normalized");
        let x = normal.x * -2.0;
        let y = normal.y * -2.0;
        let z = normal.z * -2.0;
        Self {
            mat: [
                Vector3::new(1.0 + x * normal.x, x * normal.y, x * normal.z),
                Vector3::new(y * normal.x, 1.0 + y * normal.y, y * normal.z),
                Vector3::new(z * normal.x, z * normal.y, 1.0 + z * normal.z),
            ],
        }
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

        Self {
            mat: [
                Vector3::new(x * pivot.x + 1.0, x * pivot.y, x * pivot.z),
                Vector3::new(y * pivot.x, y * pivot.y + 1.0, y * pivot.z),
                Vector3::new(z * pivot.x, z * pivot.y, z * pivot.z + 1.0),
            ],
        }
    }
}
