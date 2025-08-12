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

use super::number::{Number, SignedNumber};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vector3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: SignedNumber> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Number> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Number> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Number> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Number> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Number> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Number> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Number> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Number> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Number> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        self.as_slice().index(index)
    }
}

impl<T: Number> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);
        self.as_mut_slice().index_mut(index)
    }
}

impl<T: Number> Vector3<T> {
    /// Creates a new `Vector3` with the given x, y, and z components.
    pub fn new(x: T, y: T, z: T) -> Self {
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

    /// Returns a normalized version of this vector.
    /// If the vector is zero, it returns the vector itself.
    pub fn normalize(&self) -> Self {
        let length = self.modulus();
        if length == 0.0 {
            return *self;
        }
        let x: f64 = self.x.as_double() / length;
        let y: f64 = self.y.as_double() / length;
        let z: f64 = self.z.as_double() / length;
        Self {
            x: T::from_double(x),
            y: T::from_double(y),
            z: T::from_double(z),
        }
    }

    /// Rotates the vector around the X axis by the given angle in radians.
    pub fn rotate_x(&self, rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let y: f64 = self.y.as_double();
        let z: f64 = self.z.as_double();
        Self {
            x: self.x,
            y: T::from_double(y * cos + z * sin),
            z: T::from_double(y * sin + z * cos),
        }
    }

    /// Rotates the vector around the Y axis by the given angle in radians.
    pub fn rotate_y(&self, rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let x: f64 = self.x.as_double();
        let z: f64 = self.z.as_double();
        Self {
            x: T::from_double(x * cos + z * sin),
            y: self.y,
            z: T::from_double(x * sin + z * cos),
        }
    }

    /// Rotates the vector around the Z axis by the given angle in radians.
    pub fn rotate_z(&self, rad: f64) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();
        let x: f64 = self.x.as_double();
        let y: f64 = self.y.as_double();
        Self {
            x: T::from_double(x * cos + y * sin),
            y: T::from_double(x * sin + y * cos),
            z: self.z,
        }
    }

    /// Rotates the vector around a given axis by the specified angle in radians.
    pub fn rotate(&self, rad: f64, axis: &Self) -> Self {
        todo!()
    }

    /// Returns a slice representation of the vector.
    pub fn as_slice(&self) -> &[T; 3] {
        unsafe { std::mem::transmute(self) }
    }

    /// Returns a mutable slice representation of the vector.
    pub fn as_mut_slice(&mut self) -> &mut [T; 3] {
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
impl From<D2D_VECTOR_3F> for Vector3<f32> {
    fn from(value: D2D_VECTOR_3F) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
