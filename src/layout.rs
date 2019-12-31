#[cfg(all(
	target_arch = "x86_64",
	target_feature = "sse",
	not(target_feature = "avx")
))]
const SIMD_BYTES: usize = 16;

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
const SIMD_BYTES: usize = 32;

trait SimdRepr {
	type Repr;
	const NUM: usize;
}

/// size_of::<T>() * num == size_of::<Repr>()
impl<T> SimdRepr for T {
	default type Repr = T;
	default const NUM: usize = 1;
}

use core::arch::x86_64::__m256;

impl SimdRepr for f32 {
	type Repr = __m256;
	const NUM: usize = 8;
}

pub struct Wrap<T, const N: usize> {
	outer: Outer<T, { N / <T as SimdRepr>::NUM }>,
	leftover: LeftOvers<T, { N % <T as SimdRepr>::NUM }>,
}

#[repr(transparent)]
pub struct Outer<T, const N: usize> {
	inner: [<T as SimdRepr>::Repr; N],
}

#[repr(transparent)]
pub struct LeftOvers<T, const N: usize> {
	content: [T; N],
}
