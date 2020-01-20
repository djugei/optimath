//! build calculations in the type system, for better locality of operations
//!
//! instead of calculating the whole result for each step of the operation
//! calculate all steps of the operation for each element of the result.
//!
//! this might lead to less memory bandwidth used, as data gets worked on in one go.
//! might also lead to less cache locality tough, as elements from all inputs are used
//! instead of all elements from one (two).
//! cache locality will be slightly mitigated, as operations will (soon (TM)) run on multiple data
//! at once.

//todo: add V type that wraps a Vector and implements virtual add/mul/sub/div, just at type to get
//your calculation started

//todo: add transpose/matrix multiplication, potentially switch from basing this off off ConstIndex
//to ConstIter

use crate::consts::{ConstIndex, ConstIterator};
use core::ops::*;

pub struct VAdd<'a, 'b, T, L, R, const N: usize>
where
	// use Borrow maybe?
	// hard to abstract over lifetimes + owned/borrowd
	L: ConstIndex<&'a T, N> + Copy + Clone,
	R: ConstIndex<&'b T, N> + Copy + Clone,
	&'a T: Add<&'b T, Output = T>,
	T: 'a + 'b,
{
	l: L,
	r: R,
	m: core::marker::PhantomData<(&'a T, &'b T)>,
}

/*
impl<'a, 'b, 's, T, L, R, const N: usize> Copy for VAdd<'a, 'b, T, L, R, N>
where
	L: ConstIndex<&'a T, N> + Copy + Clone,
	R: ConstIndex<&'b T, N> + Copy + Clone,
	&'a T: Add<&'b T, Output = T>,
	T: 'a + 'b,
{
}
impl<'a, 'b, 's, T, L, R, const N: usize> Clone for VAdd<'a, 'b, T, L, R, N>
where
	L: ConstIndex<&'a T, N> + Copy + Clone,
	R: ConstIndex<&'b T, N> + Copy + Clone,
	&'a T: Add<&'b T, Output = T>,
	T: 'a + 'b,
{
	fn clone(&self) -> Self { *self }
}
// this is safe because the underlying ConstIndex implementations are guaranteed to be safe
unsafe impl<'a, 'b, 's, T, L, R, const N: usize> ConstIndex<T, N> for &'s VAdd<'a, 'b, T, L, R, N>
where
	L: ConstIndex<&'a T, N> + Copy,
	R: ConstIndex<&'b T, N> + Copy,
	&'a T: Add<&'b T, Output = T>,
	T: 'a + 'b,
{
	#[inline]
	fn i(self, index: usize) -> T {
		let l = self.l.i(index);
		let r = self.r.i(index);
		l + r
	}
}
impl<'a, 'b, 's, 'o, T, L, R, O, const N: usize> Add<&'o O> for &'s VAdd<'a, 'b, T, L, R, N>
where
	L: ConstIndex<&'a T, N> + Copy + Clone,
	R: ConstIndex<&'b T, N> + Copy + Clone,
	&'a T: Add<&'b T, Output = T>,
	T: 'a + 'b + 'o + 's,
	&'o O: ConstIndex<&'o T, N> + Copy + Clone,
{
	type Output = VAdd<'s, 'o, T, &'s VAdd<'a, 'b, T, L, R, N>, O, N>;
	fn add(self, other: O) -> Self::Output {
		VAdd {
			l: self,
			r: other,
			m: Default::default(),
		}
	}
}

impl<T, L, R, const N: usize> VAdd<T, L, R, N>
where
	L: ConstIndex<T, N> + Copy + Clone,
	R: ConstIndex<T, N> + Copy + Clone,
	T: Add<T, Output = T> + Copy + Clone,
{
	pub fn new(l: L, r: R) -> Self {
		Self {
			l,
			r,
			m: Default::default(),
		}
	}

	pub fn realize(self) -> Vector<T, N> { ConstIterator::from(self).collect() }
}

#[cfg(test)]
pub(crate) const TESTLEN: usize = 777usize;

#[test]
fn calc_chain() {
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();
	let a: Vector<f32, TESTLEN> = rng.gen();
	let b: Vector<f32, TESTLEN> = rng.gen();
	let c: Vector<f32, TESTLEN> = rng.gen();
	let d: Vector<f32, TESTLEN> = rng.gen();
	let e: Vector<f32, TESTLEN> = rng.gen();

	let ab = VAdd::new(&a, &b);
}
*/
