use crate::Vector;
use core::ops::Add;

#[cfg(all(
	target_arch = "x86_64",
	target_feature = "sse",
	not(target_feature = "avx")
))]
impl<'a, const N: usize> Add for &'a Vector<f32, N> {
	fn add(self, other: Self) -> Vector<f32, N> {
		use core::{
			arch::x86_64::{_mm_add_ps, _mm_set_ps, _mm_store_ps},
			mem::size_of,
		};

		let simd_width = 128 / 8;
		let simd_elements = simd_width / size_of::<f32>();

		let simd_self = self.inner.chunks_exact(simd_elements);
		let remainder_self = simd_self.remainder();

		let simd_other = other.inner.chunks_exact(simd_elements);
		let remainder_other = simd_other.remainder();

		let simd = simd_self
			.zip(simd_other)
			.map(|(s, o)| unsafe {
				let s = _mm_set_ps(s[3], s[2], s[1], s[0]);
				let o = _mm_set_ps(o[3], o[2], o[1], o[0]);
				let res = _mm_add_ps(s, o);
				let mut dst = [0.; 4];
				_mm_store_ps(dst.as_mut_ptr(), res);
				Vector { inner: dst }
			})
			.flatten();

		let remainder = remainder_self
			.iter()
			.zip(remainder_other)
			.map(|(s, o)| Add::add(s, o));

		simd.chain(remainder).collect()
	}
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
impl<'a, const N: usize> Add for &'a Vector<f32, N> {
	fn add(self, other: Self) -> Vector<f32, N> {
		use core::{
			arch::x86_64::{_mm256_add_ps, _mm256_set_ps, _mm256_store_ps},
			mem::size_of,
		};

		let simd_width = 256 / 8;
		let simd_elements = simd_width / size_of::<f32>();

		assert_eq!(simd_elements, 8);

		let simd_self = self.inner.chunks_exact(simd_elements);
		let remainder_self = simd_self.remainder();

		let simd_other = other.inner.chunks_exact(simd_elements);
		let remainder_other = simd_other.remainder();

		let simd = simd_self
			.zip(simd_other)
			.map(|(s, o)| unsafe {
				let s = _mm256_set_ps(s[7], s[6], s[5], s[4], s[3], s[2], s[1], s[0]);
				let o = _mm256_set_ps(o[7], o[6], o[5], o[4], o[3], o[2], o[1], o[0]);
				let res = _mm256_add_ps(s, o);
				let mut dst = [0.; 8];
				_mm256_store_ps(dst.as_mut_ptr(), res);
				Vector { inner: dst }
			})
			.flatten();

		let remainder = remainder_self
			.iter()
			.zip(remainder_other)
			.map(|(s, o)| Add::add(s, o));

		simd.chain(remainder).collect()
	}
}
