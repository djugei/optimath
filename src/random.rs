use crate::{types::Stupidity, Vector};
use rand::{
	distributions::{Distribution, Standard},
	Rng,
};

impl<T, const N: usize> Distribution<Vector<T, N>> for Standard
where
	Standard: Distribution<T>,
{
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<T, N> {
		Vector::build_with_fn(|_| rng.gen())
	}
}
