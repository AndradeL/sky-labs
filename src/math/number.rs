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

mod abs;
mod as_double;
mod wrap;

pub(crate) use self::abs::Abs;
pub(crate) use self::as_double::AsDouble;
pub(crate) use self::as_double::FromDouble;
pub use self::wrap::Wrap;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// TODO: consider making this trait const once issue is merged
// https://github.com/rust-lang/rust/issues/143874
pub trait Number:
    Sized
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + PartialEq
    + PartialOrd
    + Copy
    + Default
    + AsDouble
    + Abs
    + FromDouble
{
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_number {
    ($($t:ty)*) => ($(
        impl Number for $t {
            #[inline]
            fn zero() -> $t { 0 as $t }
            #[inline]
            fn one() -> $t { 1 as $t }
        }
    )*)
}

impl_number! { u32 u64 i32 i64 f32 f64 }

pub trait SignedNumber: Number + Neg<Output = Self> + Abs {}
impl SignedNumber for f64 {}
impl SignedNumber for f32 {}
impl SignedNumber for i64 {}
impl SignedNumber for i32 {}

pub trait SignedInteger: SignedNumber {}
impl SignedInteger for i64 {}
impl SignedInteger for i32 {}

pub trait FloatingPointNumber: SignedNumber {}
impl FloatingPointNumber for f64 {}
impl FloatingPointNumber for f32 {}
