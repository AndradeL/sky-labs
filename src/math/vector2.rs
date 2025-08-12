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
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use super::number::Number;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Vector2<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T> Neg for Vector2<T>
where
    T: Number + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Number> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Number> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Number> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Number> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Number> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Number> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Number> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Number> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Number> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);
        self.as_slice().index(index)
    }
}

impl<T: Number> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);
        self.as_mut_slice().index_mut(index)
    }
}

impl<T: Number> Vector2<T> {
    /// Creates a new `Vector2` with the given x and y components.
    pub fn new(x: T, y: T) -> Self {
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

    /// Returns a slice representation of the vector.
    pub fn as_slice(&self) -> &[T; 2] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a mutable slice representation of the vector.
    pub fn as_mut_slice(&mut self) -> &mut [T; 2] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a pointer to the vector's data.
    /// This is unsafe because it allows direct access to the vector's memory without bounds check.
    pub unsafe fn as_ptr(&self) -> *const T {
        &self.x as *const T
    }

    /// Returns a mutable pointer to the vector's data.
    /// This is unsafe because it allows direct access to the vector's memory without bounds check.
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
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
impl From<D2D_SIZE_F> for Vector2<f32> {
    fn from(value: D2D_SIZE_F) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
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
impl From<D2D_SIZE_U> for Vector2<u32> {
    fn from(value: D2D_SIZE_U) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
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
impl From<D2D_POINT_2F> for Vector2<f32> {
    fn from(value: D2D_POINT_2F) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
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
impl From<D2D_POINT_2U> for Vector2<u32> {
    fn from(value: D2D_POINT_2U) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
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
impl From<D2D_VECTOR_2F> for Vector2<f32> {
    fn from(value: D2D_VECTOR_2F) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
