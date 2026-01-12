// Copyright (c) 2026 Lucas B. Andrade
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

// Macros to simplify extending operator traits over references.
// Adapted from 'forward_ref' crate: https://crates.io/crates/forward_ref

macro_rules! forward_ref_unop {
    (impl<$gen_name:ident> $imp:ident, $method:ident for $t:ty where $($gen:tt)+) => {
        impl<$gen_name> $imp for &$t where $($gen)* {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    };
}

macro_rules! forward_ref_binop {
    (impl<$gen_name:ident> $imp:ident, $method:ident for $t:ty, $u:ty where $($gen:tt)+) => {
        impl<'a, $gen_name> $imp<$u> for &'a $t where $($gen)* {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<$gen_name> $imp<&$u> for $t where $($gen)* {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<$gen_name> $imp<&$u> for &$t where $($gen)* {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! forward_ref_op_assign {
    (impl<$gen_name:ident> $imp:ident, $method:ident for $t:ty, $u:ty where $($gen:tt)+) => {
        impl<$gen_name> $imp<&$u> for $t where $($gen)* {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    };
}

macro_rules! implement_scalar_lhs_mul {
    ($($vec_type:ty, $t:ty);+) => ($(
        impl Mul<$vec_type> for $t {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: $vec_type) -> Self::Output {
                rhs * self
            }
        }

        impl<'a> Mul<$vec_type> for &'a $t {
            type Output = <$t as Mul<$vec_type>>::Output;

            #[inline]
            fn mul(self, other: $vec_type) -> <$t as Mul<$vec_type>>::Output {
                Mul::mul(*self, other)
            }
        }

        impl Mul<&$vec_type> for $t {
            type Output = <$t as Mul<$vec_type>>::Output;

            #[inline]
            fn mul(self, other: &$vec_type) -> <$t as Mul<$vec_type>>::Output {
                Mul::mul(self, *other)
            }
        }

        impl Mul<&$vec_type> for &$t {
            type Output = <$t as Mul<$vec_type>>::Output;

            #[inline]
            fn mul(self, other: &$vec_type) -> <$t as Mul<$vec_type>>::Output {
                Mul::mul(*self, *other)
            }
        }
    )*);
}
