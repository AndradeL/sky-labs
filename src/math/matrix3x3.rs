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

use super::number::Number;
use super::Vector3;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Matrix3x3<T: Number> {
    mat: [Vector3<T>;3]
}

impl<T> Neg for Matrix3x3<T>
where
    T: Number + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            mat: 
            [
                Vector3 {
                    x: -self.mat[0][0],
                    y: -self.mat[0][1],
                    z: -self.mat[0][2], 
                },
                Vector3 {
                    x: -self.mat[1][0],
                    y: -self.mat[1][1],
                    z: -self.mat[1][2],
                },
                Vector3 {
                    x: -self.mat[2][0],
                    y: -self.mat[2][1],
                    z: -self.mat[2][2],
                },
            ]
        }
    }
}

impl<T: Number> Add for Matrix3x3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            mat: 
            [
                Vector3 {
                    x: self.mat[0][0] + rhs.mat[0][0],
                    y: self.mat[0][1] + rhs.mat[0][1],
                    z: self.mat[0][2] + rhs.mat[0][2],
                },
                Vector3 {
                    x: self.mat[1][0] + rhs.mat[1][0],
                    y: self.mat[1][1] + rhs.mat[1][1],
                    z: self.mat[1][2] + rhs.mat[1][2],
                },
                Vector3 {
                    x: self.mat[2][0] + rhs.mat[2][0],
                    y: self.mat[2][1] + rhs.mat[2][1],
                    z: self.mat[2][2] + rhs.mat[2][2],
                },
            ]
        }
    }
}

impl<T: Number> AddAssign for Matrix3x3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.mat[0][0] += rhs.mat[0][0];
        self.mat[0][1] += rhs.mat[0][1];
        self.mat[0][2] += rhs.mat[0][2];
        self.mat[1][0] += rhs.mat[1][0];
        self.mat[1][1] += rhs.mat[1][1];
        self.mat[1][2] += rhs.mat[1][2];
        self.mat[2][0] += rhs.mat[2][0];
        self.mat[2][1] += rhs.mat[2][1];
        self.mat[2][2] += rhs.mat[2][2];
    }
}

impl<T: Number> Sub for Matrix3x3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            mat: 
            [
                Vector3 {
                    x: self.mat[0][0] - rhs.mat[0][0],
                    y: self.mat[0][1] - rhs.mat[0][1],
                    z: self.mat[0][2] - rhs.mat[0][2],
                },
                Vector3 {
                    x: self.mat[1][0] - rhs.mat[1][0],
                    y: self.mat[1][1] - rhs.mat[1][1],
                    z: self.mat[1][2] - rhs.mat[1][2],
                },
                Vector3 {
                    x: self.mat[2][0] - rhs.mat[2][0],
                    y: self.mat[2][1] - rhs.mat[2][1],
                    z: self.mat[2][2] - rhs.mat[2][2],
                },
            ]
        }
    }
}

impl<T: Number> Mul<T> for Matrix3x3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            mat: 
            [
                Vector3 {
                    x: self.mat[0][0] * rhs,
                    y: self.mat[0][1] * rhs,
                    z: self.mat[0][2] * rhs,
                },
                Vector3 {
                    x: self.mat[1][0] * rhs,
                    y: self.mat[1][1] * rhs,
                    z: self.mat[1][2] * rhs,
                },
                Vector3 {
                    x: self.mat[2][0] * rhs,
                    y: self.mat[2][1] * rhs,
                    z: self.mat[2][2] * rhs,
                },
            ]
        }
    }
}

impl<T: Number> Mul<Vector3<T>> for Matrix3x3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.mat[0][0] * rhs.x + self.mat[0][1] * rhs.y + self.mat[0][2] * rhs.z,
            y: self.mat[1][0] * rhs.x + self.mat[1][1] * rhs.y + self.mat[1][2] * rhs.z,
            z: self.mat[2][0] * rhs.x + self.mat[2][1] * rhs.y + self.mat[2][2] * rhs.z,
        }
    }
}

impl<T: Number> Mul<Matrix3x3<T>> for Matrix3x3<T> {
    type Output = Self;

    fn mul(self, rhs: Matrix3x3<T>) -> Self::Output {
        Self {
            mat: 
            [
                Vector3 {
                    x: self.mat[0][0] * rhs.mat[0][0] + self.mat[0][1] * rhs.mat[1][0] + self.mat[0][2] * rhs.mat[2][0],
                    y: self.mat[0][0] * rhs.mat[0][1] + self.mat[0][1] * rhs.mat[1][1] + self.mat[0][2] * rhs.mat[2][1],
                    z: self.mat[0][0] * rhs.mat[0][2] + self.mat[0][1] * rhs.mat[1][2] + self.mat[0][2] * rhs.mat[2][2],
                },
                Vector3 {
                    x: self.mat[1][0] * rhs.mat[0][0] + self.mat[1][1] * rhs.mat[1][0] + self.mat[1][2] * rhs.mat[2][0],
                    y: self.mat[1][0] * rhs.mat[0][1] + self.mat[1][1] * rhs.mat[1][1] + self.mat[1][2] * rhs.mat[2][1],
                    z: self.mat[1][0] * rhs.mat[0][2] + self.mat[1][1] * rhs.mat[1][2] + self.mat[1][2] * rhs.mat[2][2],
                },
                Vector3 {
                    x: self.mat[2][0] * rhs.mat[0][0] + self.mat[2][1] * rhs.mat[1][0] + self.mat[2][2] * rhs.mat[2][0],
                    y: self.mat[2][0] * rhs.mat[0][1] + self.mat[2][1] * rhs.mat[1][1] + self.mat[2][2] * rhs.mat[2][1],
                    z: self.mat[2][0] * rhs.mat[0][2] + self.mat[2][1] * rhs.mat[1][2] + self.mat[2][2] * rhs.mat[2][2],
                },
            ]
        }
    }
}

impl<T: Number> MulAssign<T> for Matrix3x3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.mat[0][0] *= rhs;
        self.mat[0][1] *= rhs;
        self.mat[0][2] *= rhs;
        self.mat[1][0] *= rhs;
        self.mat[1][1] *= rhs;
        self.mat[1][2] *= rhs;
        self.mat[2][0] *= rhs;
        self.mat[2][1] *= rhs;
        self.mat[2][2] *= rhs;
    }
}

impl<T: Number> Div<T> for Matrix3x3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            mat: 
            [
                Vector3 {
                    x: self.mat[0][0] / rhs,
                    y: self.mat[0][1] / rhs,
                    z: self.mat[0][2] / rhs,
                },
                Vector3 {
                    x: self.mat[1][0] / rhs,
                    y: self.mat[1][1] / rhs,
                    z: self.mat[1][2] / rhs,
                },
                Vector3 {
                    x: self.mat[2][0] / rhs,
                    y: self.mat[2][1] / rhs,
                    z: self.mat[2][2] / rhs,
                },
            ]
        }
    }
}

impl<T: Number> DivAssign<T> for Matrix3x3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.mat[0][0] /= rhs;
        self.mat[0][1] /= rhs;
        self.mat[0][2] /= rhs;
        self.mat[1][0] /= rhs;
        self.mat[1][1] /= rhs;
        self.mat[1][2] /= rhs;
        self.mat[2][0] /= rhs;
        self.mat[2][1] /= rhs;
        self.mat[2][2] /= rhs;
    }
}

impl<T: Number> Index<usize> for Matrix3x3<T> {
    type Output = Vector3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        self.as_slice().index(index)
    }
}

impl<T:Number> IndexMut<usize> for Matrix3x3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);
        self.as_mut_slice().index_mut(index)
    }
}

impl<T: Number> Matrix3x3<T> {
    

    pub fn as_slice(&self) -> &[Vector3<T>] {
        self.mat.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Vector3<T>] {
        self.mat.as_mut_slice()
    }
}
