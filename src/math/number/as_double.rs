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

pub trait AsDouble {
    fn as_double(self) -> f64;
}

macro_rules! impl_as_double {
    ($($t:ty)*) => ($(
        impl AsDouble for $t {
            #[inline]
            fn as_double(self) -> f64 { self as f64 }
        }
    )*)
}

impl_as_double! { f64 f32 i32 i64 u32 u64 }

pub trait FromDouble {
    fn from_double(value: f64) -> Self;
}

macro_rules! impl_from_double {
    ($($t:ty)*) => ($(
        impl FromDouble for $t {
            #[inline]
            fn from_double(value: f64) -> Self { value as Self }
        }
    )*)
}

impl_from_double! { f64 f32 i32 i64 u32 u64 }
