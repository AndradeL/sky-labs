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

use crate::math::{Number, SignedNumber};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Vector2<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T: SignedNumber> Neg for Vector2<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
forward_ref_unop!(impl<T> Neg, neg for Vector2<T> where T: SignedNumber);

impl<T: Number> Add for Vector2<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
forward_ref_binop!(impl<T> Add, add for Vector2<T>, Vector2<T> where T: Number);

impl<T: Number> AddAssign for Vector2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
forward_ref_op_assign!(impl<T> AddAssign, add_assign for Vector2<T>, Vector2<T> where T: Number);

impl<T: Number> Sub for Vector2<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
forward_ref_binop!(impl<T> Sub, sub for Vector2<T>, Vector2<T> where T: Number);

impl<T: Number> SubAssign for Vector2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
forward_ref_op_assign!(impl<T> SubAssign, sub_assign for Vector2<T>, Vector2<T> where T: Number);

impl<T: Number> Mul<T> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
forward_ref_binop!(impl<T> Mul, mul for Vector2<T>, T where T: Number);
implement_scalar_lhs_mul! {
    Vector2<u32>, u32;
    Vector2<u64>, u64;
    Vector2<i32>, i32;
    Vector2<i64>, i64;
    Vector2<f32>, f32;
    Vector2<f64>, f64
}

impl<T: Number> MulAssign<T> for Vector2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
forward_ref_op_assign!(impl<T> MulAssign, mul_assign for Vector2<T>, T where T: Number);

impl<T: Number> Div<T> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
forward_ref_binop!(impl<T> Div, div for Vector2<T>, T where T: Number);

impl<T: Number> DivAssign<T> for Vector2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
forward_ref_op_assign!(impl<T> DivAssign, div_assign for Vector2<T>, T where T: Number);

impl<'a, T: Number> From<&'a [T]> for Vector2<T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        debug_assert!(slice.len() >= 2, "Slice must have at least 2 elements");
        Self {
            x: slice[0],
            y: slice[1],
        }
    }
}

impl<'a, T: Number> From<&'a [T]> for &'a Vector2<T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        debug_assert!(slice.len() >= 2, "Slice must have at least 2 elements");
        unsafe { std::mem::transmute(&slice[0]) }
    }
}

impl<T: Number> From<[T; 2]> for Vector2<T> {
    #[inline]
    fn from(array: [T; 2]) -> Self {
        Self {
            x: array[0],
            y: array[1],
        }
    }
}

impl<T: Number> Index<usize> for Vector2<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);
        self.as_slice().index(index)
    }
}

impl<T: Number> IndexMut<usize> for Vector2<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);
        self.as_mut_slice().index_mut(index)
    }
}

impl<T: Number> Vector2<T> {
    /// Creates a new `Vector2` with the given x and y components.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns a default `Vector2` with both components set to zero.
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    /// Returns a `Vector2` with both components set to one.
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
        }
    }

    /// Returns the modulus (length) of the vector.
    pub fn modulus(&self) -> f64 {
        let origin = Vector2::default();
        self.distance_to(&origin)
    }

    /// Returns the magnitude (norm) of the vector, the same as modulus().
    pub fn magnitude(&self) -> f64 {
        self.modulus()
    }

    /// Returns the squared norm of the vector.
    /// This is useful for avoiding the square root operation when comparing distances.
    pub fn norm_squared(&self) -> T {
        let x = self.x;
        let y = self.y;
        x * x + y * y
    }

    /// Returns the distance to another vector.
    pub fn distance_to(&self, other: &Vector2<T>) -> f64 {
        let diff = *self - *other;
        let norm_squared: f64 = diff.norm_squared().as_double();
        f64::sqrt(norm_squared)
    }

    /// Returns the taxicab distance (Manhattan distance) to another vector.
    pub fn taxicab_distance(&self, other: Vector2<T>) -> T {
        T::abs(self.x - other.x) + T::abs(self.y - other.y)
    }

    /// Returns the dot product of this vector with another vector.
    pub fn dot(&self, other: Vector2<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    /// Returns a normalized version of the vector.
    /// If the vector is zero, it returns the original vector.
    pub fn normalize(&self) -> Self {
        let length = self.modulus();
        if length == 0.0 {
            return *self;
        }
        let x: f64 = self.x.as_double() / length;
        let y: f64 = self.y.as_double() / length;
        Self {
            x: T::from_double(x),
            y: T::from_double(y),
        }
    }

    /// Rotates the vector around the origin by the given angle in radians.
    /// The rotation is counter-clockwise.
    pub fn rotate(&self, rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let x: f64 = self.x.as_double();
        let y: f64 = self.y.as_double();
        Self {
            x: T::from_double(x * cos + y * sin),
            y: T::from_double(x * sin + y * cos),
        }
    }

    pub const fn from_array(arr: [T; 2]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
        }
    }

    pub const fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        debug_assert!(slice.len() >= 2, "Slice must have at least 2 elements");
        Self {
            x: slice[0],
            y: slice[1],
        }
    }

    /// Returns a slice representation of the vector.
    pub const fn as_slice(&self) -> &[T; 2] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a mutable slice representation of the vector.
    pub const fn as_mut_slice(&mut self) -> &mut [T; 2] {
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

/// Windows-specific implementation for Direct2D compatibility.

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_POINT_2F, D2D_POINT_2U, D2D_SIZE_F, D2D_SIZE_U, D2D_VECTOR_2F,
};

#[cfg(target_os = "windows")]
impl Into<D2D_SIZE_F> for Vector2<f32> {
    fn into(self) -> D2D_SIZE_F {
        D2D_SIZE_F {
            width: self.x,
            height: self.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> Into<&'a D2D_SIZE_F> for &'a Vector2<f32> {
    fn into(self) -> &'a D2D_SIZE_F {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_SIZE_F> for Vector2<f32> {
    fn from(value: D2D_SIZE_F) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> From<&'a D2D_SIZE_F> for &'a Vector2<f32> {
    fn from(value: &'a D2D_SIZE_F) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(target_os = "windows")]
impl Into<D2D_SIZE_U> for Vector2<u32> {
    fn into(self) -> D2D_SIZE_U {
        D2D_SIZE_U {
            width: self.x,
            height: self.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> Into<&'a D2D_SIZE_U> for &'a Vector2<u32> {
    fn into(self) -> &'a D2D_SIZE_U {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_SIZE_U> for Vector2<u32> {
    fn from(value: D2D_SIZE_U) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> From<&'a D2D_SIZE_U> for &'a Vector2<u32> {
    fn from(value: &'a D2D_SIZE_U) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(target_os = "windows")]
impl Into<D2D_POINT_2F> for Vector2<f32> {
    fn into(self) -> D2D_POINT_2F {
        D2D_POINT_2F {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> Into<&'a D2D_POINT_2F> for &'a Vector2<f32> {
    fn into(self) -> &'a D2D_POINT_2F {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_POINT_2F> for Vector2<f32> {
    fn from(value: D2D_POINT_2F) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> From<&'a D2D_POINT_2F> for &'a Vector2<f32> {
    fn from(value: &'a D2D_POINT_2F) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(target_os = "windows")]
impl Into<D2D_POINT_2U> for Vector2<u32> {
    fn into(self) -> D2D_POINT_2U {
        D2D_POINT_2U {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> Into<&'a D2D_POINT_2U> for &'a Vector2<u32> {
    fn into(self) -> &'a D2D_POINT_2U {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_POINT_2U> for Vector2<u32> {
    fn from(value: D2D_POINT_2U) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> From<&'a D2D_POINT_2U> for &'a Vector2<u32> {
    fn from(value: &'a D2D_POINT_2U) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(target_os = "windows")]
impl Into<D2D_VECTOR_2F> for Vector2<f32> {
    fn into(self) -> D2D_VECTOR_2F {
        D2D_VECTOR_2F {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> Into<&'a D2D_VECTOR_2F> for &'a Vector2<u32> {
    fn into(self) -> &'a D2D_VECTOR_2F {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_VECTOR_2F> for Vector2<f32> {
    fn from(value: D2D_VECTOR_2F) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(target_os = "windows")]
impl<'a> From<&'a D2D_VECTOR_2F> for &'a Vector2<f32> {
    fn from(value: &'a D2D_VECTOR_2F) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
