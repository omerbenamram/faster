// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use stdsimd::vendor::*;
use vecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};

pub trait Recip {
    /// Return a vector containing an estimation of the reciprocal of the
    /// corresponding elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert!(0.25 - 0.01 < f32s(4.0).recip().coalesce() &&
    ///         0.25 + 0.01 > f32s(4.0).recip().coalesce());
    /// # }
    /// ```
    fn recip(&self) -> Self;
}

rust_fallback_impl! {
    impl Recip for f32x8 where "avx" {
        recip => _mm256_rcp_ps(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_impl! {
    impl Recip for f32x4 where "sse" {
        recip => _mm_rcp_ps(), [0, 1, 2, 3];
    }
}

#[cfg(test)]
mod tests {
    use vecs::*;
    use intrin::*;
    use core_or_std::f32::INFINITY;

    #[test]
    fn recip_f32s() {
        let mut i = -1024.0;
        while i < 1024.0 {
            // This test has some pretty significant float error if done on x86
            let ans = f32s(i).recip().extract(0);
            let real = f32s(1.0 / i).extract(0);
            assert!((real == INFINITY && ans == INFINITY) || (ans - real).abs() < 0.0005);
            i += 1.0
        }
    }
}
