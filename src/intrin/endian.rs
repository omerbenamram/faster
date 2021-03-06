use vecs::*;
use stdsimd::vendor::*;
use intrin::transmute::*;
use core_or_std::mem::transmute;

pub trait Reendianize : Sized + Copy {
    /// Return a vector containing elements of `self` with switched endianness.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u32s(0xDEADBEEF).swap_bytes(), u32s(0xEFBEADDE));
    /// # }
    /// ```
    fn swap_bytes(&self) -> Self;

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_be(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_be(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_le(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_le(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn from_be(&self) -> Self {
        *self
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn from_be(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn from_le(&self) -> Self {
        self.swap_bytes()
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn from_le(&self) -> Self {
        *self
    }
}

macro_rules! impl_packed_swap_bytes {
    ($vec:tt, $uvec:tt, $feat:expr, $mmfn:tt, ($($c:expr),*), ($($a:expr, $b:expr),*)) => {
        impl Reendianize for $vec {
            #[cfg(not(target_feature = $feat))]
            #[inline(always)]
            fn swap_bytes(&self) -> Self {
                $vec::new($(self.extract($a).swap_bytes(),
                            self.extract($b).swap_bytes()),*)
            }

            #[cfg(target_feature = $feat)]
            #[inline(always)]
            fn swap_bytes(&self) -> Self {
                unsafe {
                    transmute($mmfn(self.be_u8s(), $uvec::new($($c),*).be_u8s()))
                }
            }
        }
    }
}

impl_packed_swap_bytes!(u8x64, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63));
impl_packed_swap_bytes!(u8x32, u8x32, "avx2", _mm256_shuffle_epi8,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swap_bytes!(u8x16, u8x16, "ssse3", _mm_shuffle_epi8,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(i8x64, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63));
impl_packed_swap_bytes!(i8x32, u8x32, "avx2", _mm256_shuffle_epi8,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swap_bytes!(i8x16, u8x16, "ssse3", _mm_shuffle_epi8,
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(u16x32, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30, 33, 32, 35, 34, 37, 36, 39, 38, 41, 40, 43, 42, 45, 44, 47, 46, 49, 48, 51, 50, 53, 52, 55, 54, 57, 56, 59, 58, 61, 60, 63, 62),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swap_bytes!(u16x16, u8x32, "avx2", _mm256_shuffle_epi8,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(u16x8, u8x16, "ssse3", _mm_shuffle_epi8,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(i16x32, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30, 33, 32, 35, 34, 37, 36, 39, 38, 41, 40, 43, 42, 45, 44, 47, 46, 49, 48, 51, 50, 53, 52, 55, 54, 57, 56, 59, 58, 61, 60, 63, 62),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31));
impl_packed_swap_bytes!(i16x16, u8x32, "avx2", _mm256_shuffle_epi8,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(i16x8, u8x16, "ssse3", _mm_shuffle_epi8,
                        (1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(u32x16, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28, 35, 34, 33, 32, 39, 38, 37, 36, 43, 42, 41, 40, 47, 46, 45, 44, 51, 50, 49, 48, 55, 54, 53, 52, 59, 58, 57, 56, 63, 62, 61, 60),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(u32x8, u8x32, "avx2", _mm256_shuffle_epi8,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(u32x4, u8x16, "ssse3", _mm_shuffle_epi8,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12),
                        (0, 1, 2, 3));
impl_packed_swap_bytes!(i32x16, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28, 35, 34, 33, 32, 39, 38, 37, 36, 43, 42, 41, 40, 47, 46, 45, 44, 51, 50, 49, 48, 55, 54, 53, 52, 59, 58, 57, 56, 63, 62, 61, 60),
                        (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
impl_packed_swap_bytes!(i32x8, u8x32, "avx2", _mm256_shuffle_epi8,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(i32x4, u8x16, "ssse3", _mm_shuffle_epi8,
                        (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12),
                        (0, 1, 2, 3));
impl_packed_swap_bytes!(u64x8, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24, 39, 38, 37, 36, 35, 34, 33, 32, 47, 46, 45, 44, 43, 42, 41, 40, 55, 54, 53, 52, 51, 50, 49, 48, 63, 62, 61, 60, 59, 58, 57, 56),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(u64x4, u8x32, "avx2", _mm256_shuffle_epi8,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24),
                        (0, 1, 2, 3));
impl_packed_swap_bytes!(u64x2, u8x16, "ssse3", _mm_shuffle_epi8,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8),
                        (0, 1));
impl_packed_swap_bytes!(i64x8, u8x64, "avx512-butnotyet", _mm512_permutexvar_epi8,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24, 39, 38, 37, 36, 35, 34, 33, 32, 47, 46, 45, 44, 43, 42, 41, 40, 55, 54, 53, 52, 51, 50, 49, 48, 63, 62, 61, 60, 59, 58, 57, 56),
                        (0, 1, 2, 3, 4, 5, 6, 7));
impl_packed_swap_bytes!(i64x4, u8x32, "avx2", _mm256_shuffle_epi8,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24),
                        (0, 1, 2, 3));
impl_packed_swap_bytes!(i64x2, u8x16, "ssse3", _mm_shuffle_epi8,
                        (7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8),
                        (0, 1));


mod tests {
    use super::*;

    macro_rules! test_packed_swap_bytes {
        (($($vec:tt),*), ($($fn:tt),*)) => {
            $(
                #[test]
                fn $fn() {
                    let a = $vec::interleave(33u8 as <$vec as Packed>::Scalar,
                                             92u8 as <$vec as Packed>::Scalar);
                    let b = $vec::interleave((33u8 as <$vec as Packed>::Scalar).swap_bytes(),
                                             (92u8 as <$vec as Packed>::Scalar).swap_bytes());
                    assert_eq!(a.swap_bytes(), b);
                }
            )*
        }
    }

    test_packed_swap_bytes!((u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2),
                             (swap_bytes_u8x64, swap_bytes_u8x32, swap_bytes_u8x16, swap_bytes_i8x64, swap_bytes_i8x32, swap_bytes_i8x16, swap_bytes_u16x32, swap_bytes_u16x16, swap_bytes_u16x8, swap_bytes_i16x32, swap_bytes_i16x16, swap_bytes_i16x8, swap_bytes_u32x16, swap_bytes_u32x8, swap_bytes_u32x4, swap_bytes_i32x16, swap_bytes_i32x8, swap_bytes_i32x4, swap_bytes_u64x8, swap_bytes_u64x4, swap_bytes_u64x2, swap_bytes_i64x8, swap_bytes_i64x4, swap_bytes_i64x2));
}
