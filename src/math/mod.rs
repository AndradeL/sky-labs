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

mod matrix3x3;
mod matrix4x4;
mod number;
mod rect;
mod size;
mod vector2;
mod vector3;
mod vector4;

pub use self::matrix3x3::Matrix3x3;
pub use self::matrix4x4::Matrix4x4;
pub use self::number::Wrap;
pub(crate) use self::number::{Number, SignedNumber};
pub use self::rect::Rect;
pub use self::size::Size;
pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
