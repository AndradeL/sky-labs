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

use super::number::Number;
use super::Vector2;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Size<T: Number> {
    pub width: T,
    pub height: T,
}

impl<T> From<Vector2<T>> for Size<T> where T : Number {
    fn from(point: Vector2<T>) -> Self {
        Self {
            width: point.x,
            height: point.y,
        }
    }
}

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct2D::Common::D2D_SIZE_F;


#[cfg(target_os = "windows")]
impl Into<D2D_SIZE_F> for Size<f32> {
    fn into(self) -> D2D_SIZE_F {
        D2D_SIZE_F {
            width: self.width,
            height: self.height,
        }
    }
}

#[cfg(target_os = "windows")]
impl From<D2D_SIZE_F> for Size<f32> {
    fn from(value: D2D_SIZE_F) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
