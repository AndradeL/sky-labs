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
use std::slice;

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

impl<T:Number> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);
        self.as_mut_slice().index_mut(index)
    }
}

impl<T: Number> Vector2<T> {
    pub fn modulus(&self) -> f64 {
        let origin = Vector2::default();
        self.distance_to(&origin)
    }

    pub fn distance_to(&self, b: &Vector2<T>) -> f64 {
        let x: f64 = (self.x - b.x).as_double();
        let y: f64 = (self.y - b.y).as_double();
        f64::sqrt(x * x + y * y)
    }

    pub fn taxicab_distance(&self, b: Vector2<T>) -> T {
        T::abs(self.x - b.x) + T::abs(self.y - b.y)
    }

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

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), 2) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), 2) }
    }

    unsafe fn as_ptr(&self) -> *const T {
        &self.x as *const T
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.x as *mut T
    }
}

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct2D::Common::D2D_SIZE_F;

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

