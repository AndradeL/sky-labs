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

impl AsDouble for f64 {
    fn as_double(self) -> f64 {
        self
    }
}
impl AsDouble for f32 {
    fn as_double(self) -> f64 {
        self as f64
    }
}
impl AsDouble for i32 {
    fn as_double(self) -> f64 {
        self as f64
    }
}
impl AsDouble for i64 {
    fn as_double(self) -> f64 {
        self as f64
    }
}
impl AsDouble for u32 {
    fn as_double(self) -> f64 {
        self as f64
    }
}

impl AsDouble for u64 {
    fn as_double(self) -> f64 {
        self as f64
    }
}

pub trait FromDouble {
    fn from_double(value: f64) -> Self;
}

impl FromDouble for f64 {
    fn from_double(value: f64) -> Self {
        value as Self
    }
}
impl FromDouble for f32 {
    fn from_double(value: f64) -> Self {
        value as Self
    }
}
impl FromDouble for i32 {
    fn from_double(value: f64) -> Self {
        value as Self
    }
}
impl FromDouble for i64 {
    fn from_double(value: f64) -> Self {
        value as Self
    }
}
impl FromDouble for u32 {
    fn from_double(value: f64) -> Self {
        value as Self
    }
}

impl FromDouble for u64 {
    fn from_double(value: f64) -> Self {
        value as Self
    }
}