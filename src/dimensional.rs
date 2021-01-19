#![allow(dead_code)]
use core::marker::PhantomData;

enum One<const N: usize> {}

// would love for this to be an enum, rust issue #32739
struct Multi<T: Dimension, const N: usize> {
	marker: PhantomData<T>,
}

trait Dimension {
	const MUL: usize;
	const DIMS: usize;
}

impl<const N: usize> Dimension for One<N> {
	const MUL: usize = N;
	const DIMS: usize = 1;
}

impl<T: Dimension, const N: usize> Dimension for Multi<T, N> {
	const MUL: usize = N * T::MUL;
	const DIMS: usize = T::DIMS + 1;
}

struct Mathable<T, Dim: Dimension> {
	// cool, so ice fixed, but by removing functionality.
	// I do indeed want to depend on the generic parameter.
	// that is kind of the point.
	inner: [T; Dim::MUL],
	marker: PhantomData<Dim>,
}
