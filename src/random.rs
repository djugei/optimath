use crate::Vector;
use core::mem::MaybeUninit;
use rand::{
	distributions::{Distribution, Standard},
	Rng,
};

impl<T, const N: usize> Distribution<Vector<T, N>> for Standard
where
	Standard: Distribution<T>,
{
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<T, N> {
		let mut inner: MaybeUninit<[T; N]> = Vector::uninit_inner();
		let base_ptr = inner.as_mut_ptr() as *mut T;
		// TODO: add from_filler_function that takes a closure
		for offset in 0..N {
			// can't overshoot cause N is const generic
			unsafe { base_ptr.add(offset).write(rng.gen()) }
		}

		// all elements have been initialized at this point cause N is const generic
		unsafe {
			Vector {
				inner: inner.assume_init(),
			}
		}
	}
}
