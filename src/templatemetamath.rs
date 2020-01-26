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

pub struct VAdd<T, L, R, LT, RT, const N: usize>
where
	// use Borrow maybe?
	// hard to abstract over lifetimes + owned/borrowd
	L: ConstIndex<LT, N> + Copy + Clone,
	R: ConstIndex<RT, N> + Copy + Clone,
	LT: Add<RT, Output = T>,
{
	l: L,
	r: R,
	m: core::marker::PhantomData<(T, LT, RT)>,
}

impl<T, L, R, LT, RT, const N: usize> Copy for VAdd<T, L, R, LT, RT, N>
where
	L: ConstIndex<LT, N> + Copy + Clone,
	R: ConstIndex<RT, N> + Copy + Clone,
	LT: Add<RT, Output = T>,
{
}

impl<T, L, R, LT, RT, const N: usize> Clone for VAdd<T, L, R, LT, RT, N>
where
	L: ConstIndex<LT, N> + Copy + Clone,
	R: ConstIndex<RT, N> + Copy + Clone,
	LT: Add<RT, Output = T>,
{
	fn clone(&self) -> Self { *self }
}

// this is safe because the underlying ConstIndex implementations are guaranteed to be safe
unsafe impl<T, L, R, LT, RT, const N: usize> ConstIndex<T, N> for VAdd<T, L, R, LT, RT, N>
where
	L: ConstIndex<LT, N> + Copy + Clone,
	R: ConstIndex<RT, N> + Copy + Clone,
	LT: Add<RT, Output = T>,
{
	#[inline]
	fn i(self, index: usize) -> T {
		let l = self.l.i(index);
		let r = self.r.i(index);
		l + r
	}
}

// this restricts other to const-Index to the same type as self
//
// this is not a necessary restriction, but rust type system does not allow for expressing anything
// more generic due to "unconstrained type parameters"
//
// this might be possible once GAT lands, allowing for stuff like (X,Y) + (Z,) = (X, Y, Z) or
// the like. like for example T + &T which is kinda important...
impl<T, L, R, LT, RT, O, NT, const N: usize> Add<O> for VAdd<T, L, R, LT, RT, N>
where
	L: ConstIndex<LT, N> + Copy + Clone,
	R: ConstIndex<RT, N> + Copy + Clone,
	LT: Add<RT, Output = T>,

	O: ConstIndex<T, N> + Copy + Clone,
	T: Add<T, Output = NT>,
{
	type Output = VAdd<NT, Self, O, T, T, N>;
	fn add(self, other: O) -> Self::Output {
		VAdd {
			l: self,
			r: other,
			m: Default::default(),
		}
	}
}

/*
// can't even specialize for vector, cause "downstream crates may implement ConstIndex
// except im already implementing that in this crate...
use crate::Vector;
impl<'o, T, L, R, LT, RT, NT, const N: usize> Add<&'o Vector<T, N>> for VAdd<T, L, R, LT, RT, N>
where
	L: ConstIndex<LT, N> + Copy + Clone,
	R: ConstIndex<RT, N> + Copy + Clone,
	LT: Add<RT, Output = T>,

	T: Add<&'o T, Output = NT>,
{
	type Output = VAdd<NT, Self, &'o Vector<T, N>, T, T, N>;
	fn add(self, other: &'o Vector<T, N>) -> Self::Output {
		VAdd {
			l: self,
			r: other,
			m: Default::default(),
		}
	}
}
*/

/*
impl<T, L, R, LT, RT, const N: usize> VAdd<T, L, R, LT, RT, N>
where
	L: ConstIndex<LT, N> + Copy + Clone,
	R: ConstIndex<RT, N> + Copy + Clone,
	LT: Add<RT, Output = T>,
{
	pub fn new(l: L, r: R) -> Self {
		Self {
			l,
			r,
			m: Default::default(),
		}
	}

	pub fn realize(self) -> crate::Vector<T, N> { ConstIterator::from(self).collect() }
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
