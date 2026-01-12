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

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::math::number::{Number, SignedNumber};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vector3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: SignedNumber> Neg for Vector3<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
forward_ref_unop!(impl<T> Neg, neg for Vector3<T> where T: SignedNumber);

impl<T: Number> Add for Vector3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
forward_ref_binop!(impl<T> Add, add for Vector3<T>, Vector3<T> where T: Number);

impl<T: Number> AddAssign for Vector3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
forward_ref_op_assign!(impl<T> AddAssign, add_assign for Vector3<T>, Vector3<T> where T: Number);

impl<T: Number> Sub for Vector3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
forward_ref_binop!(impl<T> Sub, sub for Vector3<T>, Vector3<T> where T: Number);

impl<T: Number> SubAssign for Vector3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
forward_ref_op_assign!(impl<T> SubAssign, sub_assign for Vector3<T>, Vector3<T> where T: Number);

impl<T: Number> Mul<T> for Vector3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
forward_ref_binop!(impl<T> Mul, mul for Vector3<T>, T where T: Number);
implement_scalar_lhs_mul! {
    Vector3<u32>, u32;
    Vector3<u64>, u64;
    Vector3<i32>, i32;
    Vector3<i64>, i64;
    Vector3<f32>, f32;
    Vector3<f64>, f64
}

impl<T: Number> MulAssign<T> for Vector3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
forward_ref_op_assign!(impl<T> MulAssign, mul_assign for Vector3<T>, T where T: Number);

impl<T: Number> Div<T> for Vector3<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
forward_ref_binop!(impl<T> Div, div for Vector3<T>, T where T: Number);

impl<T: Number> DivAssign<T> for Vector3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
forward_ref_op_assign!(impl<T> DivAssign, div_assign for Vector3<T>, T where T: Number);

impl<T: Number> From<&[T]> for Vector3<T> {
    #[inline]
    fn from(slice: &[T]) -> Self {
        debug_assert!(slice.len() >= 3, "Slice must have at least 3 elements");
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl<'a, T: Number> From<&'a [T]> for &'a Vector3<T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        debug_assert!(slice.len() >= 3, "Slice must have at least 3 elements");
        unsafe { std::mem::transmute(&slice[0]) }
    }
}

impl<T: Number> From<[T; 3]> for Vector3<T> {
    #[inline]
    fn from(array: [T; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<T: Number> Index<usize> for Vector3<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        self.as_slice().index(index)
    }
}

impl<T: Number> IndexMut<usize> for Vector3<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);
        self.as_mut_slice().index_mut(index)
    }
}

impl<T: Number> Vector3<T> {
    /// Creates a new `Vector3` with the given x, y, and z components.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Returns a zero vector.
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Returns a vector with all components set to one.
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }

    /// Returns the modulus (length) of the vector.
    pub fn modulus(&self) -> f64 {
        let origin = Self::default();
        self.distance_to(&origin)
    }

    /// Returns the magnitude (norm) of the vector, same as modulus().
    pub fn magnitude(&self) -> f64 {
        self.modulus()
    }

    /// Returns the squared norm of the vector.
    /// This is useful for avoiding the square root operation when comparing distances.
    pub fn norm_squared(&self) -> T {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        x * x + y * y + z * z
    }

    /// Returns the distance to another vector.
    /// This is the Euclidean distance between the two vectors.
    pub fn distance_to(&self, other: &Self) -> f64 {
        let diff = *self - *other;
        let norm_squared = diff.norm_squared().as_double();
        f64::sqrt(norm_squared)
    }

    /// Returns the taxicab distance (Manhattan distance) to another vector.
    pub fn taxicab_distance(&self, other: &Self) -> T {
        T::abs(self.x - other.x) + T::abs(self.y - other.y) + T::abs(self.z - other.z)
    }

    /// Returns the cross product of this vector with another vector.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the dot product of this vector with another vector.
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn from_array(arr: [T; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }

    pub const fn to_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        debug_assert!(slice.len() >= 3, "Slice must have at least 3 elements");
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    /// Returns a slice representation of the vector.
    pub const fn as_slice(&self) -> &[T; 3] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a mutable slice representation of the vector.
    pub const fn as_mut_slice(&mut self) -> &mut [T; 3] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a pointer to the vector's data.
    /// This is unsafe because it allows direct access to the vector's memory without bounds check.
    pub const unsafe fn as_ptr(&self) -> *const T {
        &self.x as *const T
    }

    /// Returns a mutable pointer to the vector's data.
    /// This is unsafe because it allows direct access to the vector's memory without bounds check.
    pub const unsafe fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.x as *mut T
    }
}

impl Vector3<f32> {
    /// Rotates the vector around the X axis by the given angle in radians.
    pub fn rotate_x(&self, rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let y = self.y;
        let z = self.z;
        Self {
            x: self.x,
            y: y * cos - z * sin,
            z: y * sin + z * cos,
        }
    }

    /// Rotates the vector around the Y axis by the given angle in radians.
    pub fn rotate_y(&self, rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.x;
        let z = self.z;
        Self {
            x: x * cos - z * sin,
            y: self.y,
            z: x * sin + z * cos,
        }
    }

    /// Rotates the vector around the Z axis by the given angle in radians.
    pub fn rotate_z(&self, rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.x;
        let y = self.y;
        Self {
            x: x * cos - y * sin,
            y: x * sin + y * cos,
            z: self.z,
        }
    }

    /// Rotates the vector around a given axis by the specified angle in radians.
    pub fn rotate(&self, rad: f32, axis: &Self) -> Self {
        let parallel_part = *axis * self.dot(axis);
        let orthogonal_part = axis.cross(self);
        let rejection = *self - parallel_part;

        let cos = rad.cos();
        let sin = rad.sin();

        parallel_part + rejection * cos + orthogonal_part * sin
    }

    /// Returns a normalized version of this vector.
    /// If the vector is zero, it returns the vector itself.
    pub fn normalize(&self) -> Self {
        let length = self.modulus();
        if length == 0.0 {
            return *self;
        }

        *self / length as f32
    }

    /// Checks if the vector is normalized (length is 1).
    pub fn is_normalized(&self) -> bool {
        let length_squared = self.norm_squared();
        let diff = (length_squared - 1.0).abs();
        (diff * diff) <= f32::EPSILON
    }
}

impl Vector3<f64> {
    /// Rotates the vector around the X axis by the given angle in radians.
    pub fn rotate_x(&self, rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let y = self.y;
        let z = self.z;
        Self {
            x: self.x,
            y: y * cos - z * sin,
            z: y * sin + z * cos,
        }
    }

    /// Rotates the vector around the Y axis by the given angle in radians.
    pub fn rotate_y(&self, rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.x;
        let z = self.z;
        Self {
            x: x * cos - z * sin,
            y: self.y,
            z: x * sin + z * cos,
        }
    }

    /// Rotates the vector around the Z axis by the given angle in radians.
    pub fn rotate_z(&self, rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.x;
        let y = self.y;
        Self {
            x: x * cos - y * sin,
            y: x * sin + y * cos,
            z: self.z,
        }
    }

    /// Rotates the vector around a given axis by the specified angle in radians.
    pub fn rotate(&self, rad: f64, axis: &Self) -> Self {
        let parallel_part = *axis * self.dot(axis);
        let orthogonal_part = axis.cross(self);
        let rejection = *self - parallel_part;

        let cos = rad.cos();
        let sin = rad.sin();

        parallel_part + rejection * cos + orthogonal_part * sin
    }

    /// Returns a normalized version of this vector.
    /// If the vector is zero, it returns the vector itself.
    pub fn normalize(&self) -> Self {
        let length = self.modulus();
        if length == 0.0 {
            return *self;
        }

        *self / length
    }

    /// Checks if the vector is normalized (length is 1).
    pub fn is_normalized(&self) -> bool {
        let length_squared = self.norm_squared();
        let diff = (length_squared - 1.0).abs();
        (diff * diff) <= f64::EPSILON
    }
}

// Windows-specific implementation for Direct2D compatibility.

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct2D::Common::D2D_VECTOR_3F;

#[cfg(target_os = "windows")]
impl Into<D2D_VECTOR_3F> for Vector3<f32> {
    fn into(self) -> D2D_VECTOR_3F {
        D2D_VECTOR_3F {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> Into<&'a D2D_VECTOR_3F> for &'a Vector3<f32> {
    fn into(self) -> &'a D2D_VECTOR_3F {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_VECTOR_3F> for Vector3<f32> {
    fn from(value: D2D_VECTOR_3F) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> From<&'a D2D_VECTOR_3F> for &'a Vector3<f32> {
    fn from(value: &'a D2D_VECTOR_3F) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
