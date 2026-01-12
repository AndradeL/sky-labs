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

use crate::math::{Number, SignedNumber, Vector3};

/// A 4D vector with generic number type.
/// It can be used for various mathematical operations such as addition, subtraction, multiplication, and division.
/// It also provides methods for negation, indexing, and conversion to and from slices.
/// It is designed to be efficient and flexible, allowing for easy manipulation of 4D vectors in mathematical computations.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vector4<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: SignedNumber> Neg for Vector4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
forward_ref_unop!(impl<T> Neg, neg for Vector4<T> where T: SignedNumber);

impl<T: Number> Add for Vector4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
forward_ref_binop!(impl<T> Add, add for Vector4<T>, Vector4<T> where T: Number);

impl<T: Number> AddAssign for Vector4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
forward_ref_op_assign!(impl<T> AddAssign, add_assign for Vector4<T>, Vector4<T> where T: Number);

impl<T: Number> Sub for Vector4<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
forward_ref_binop!(impl<T> Sub, sub for Vector4<T>, Vector4<T> where T: Number);

impl<T: Number> SubAssign for Vector4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
forward_ref_op_assign!(impl<T> SubAssign, sub_assign for Vector4<T>, Vector4<T> where T: Number);

impl<T: Number> Mul<T> for Vector4<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
forward_ref_binop!(impl<T> Mul, mul for Vector4<T>, T where T: Number);
implement_scalar_lhs_mul! {
    Vector4<u32>, u32;
    Vector4<u64>, u64;
    Vector4<i32>, i32;
    Vector4<i64>, i64;
    Vector4<f32>, f32;
    Vector4<f64>, f64
}

impl<T: Number> MulAssign<T> for Vector4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}
forward_ref_op_assign!(impl<T> MulAssign, mul_assign for Vector4<T>, T where T: Number);

impl<T: Number> Div<T> for Vector4<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
forward_ref_binop!(impl<T> Div, div for Vector4<T>, T where T: Number);

impl<T: Number> DivAssign<T> for Vector4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}
forward_ref_op_assign!(impl<T> DivAssign, div_assign for Vector4<T>, T where T: Number);

impl<T: Number> Index<usize> for Vector4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);
        self.as_slice().index(index)
    }
}

impl<T: Number> IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);
        self.as_mut_slice().index_mut(index)
    }
}

impl<T: Number> From<&[T]> for Vector4<T> {
    #[inline]
    fn from(slice: &[T]) -> Self {
        debug_assert!(slice.len() >= 4, "Slice must have at least 4 elements");
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl<'a, T: Number> From<&'a [T]> for &'a Vector4<T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        debug_assert!(slice.len() >= 4, "Slice must have at least 4 elements");
        unsafe { std::mem::transmute(&slice[0]) }
    }
}

impl<T: Number> From<[T; 4]> for Vector4<T> {
    #[inline]
    fn from(array: [T; 4]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
    }
}

impl<T: Number> Vector4<T> {
    /// Creates a new vector with the specified components.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a new vector with all components set to zero.
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    /// Creates a new vector with all components set to one.
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
            w: T::one(),
        }
    }

    /// Converts a `Vector3` to a `Vector4` by adding the `w` component.
    pub fn from_vector3(vector: &Vector3<T>, w: T) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w,
        }
    }

    /// Returns the dot product of this vector with another vector.
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub const fn from_array(arr: [T; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }

    pub const fn to_array(&self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        debug_assert!(slice.len() >= 4, "Slice must have at least 4 elements");
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }

    /// Returns a slice representation of the vector.
    pub const fn as_slice(&self) -> &[T; 4] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a mutable slice representation of the vector.
    pub const fn as_mut_slice(&mut self) -> &mut [T; 4] {
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

// Windows-specific implementation for Direct2D compatibility

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct2D::Common::D2D_VECTOR_4F;

#[cfg(target_os = "windows")]
impl Into<D2D_VECTOR_4F> for Vector4<f32> {
    fn into(self) -> D2D_VECTOR_4F {
        D2D_VECTOR_4F {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> Into<&'a D2D_VECTOR_4F> for &'a Vector4<f32> {
    fn into(self) -> &'a D2D_VECTOR_4F {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_VECTOR_4F> for Vector4<f32> {
    fn from(value: D2D_VECTOR_4F) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> From<&'a D2D_VECTOR_4F> for &'a Vector4<f32> {
    fn from(value: &'a D2D_VECTOR_4F) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
