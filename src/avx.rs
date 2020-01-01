const SIMD_BYTES: usize = 32;

use core::arch::x86_64::__m256;

impl SimdRepr for f32 {
	type Repr = __m256;
	const NUM: usize = 8;
}
