use crate::layout::SimdRepr;
use core::arch::x86_64::__m128;

const SIMD_BYTES: usize = 16;

/*
impl SimdRepr for f32 {
	type Repr = __m128;
	const NUM: usize = 4;
    fn pack(un: Self::Unpacked) -> Self::Repr {
    }
}
*/

