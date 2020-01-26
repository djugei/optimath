use crate::{ConstIndex, Stupidity, Vector};

const S: usize = 250;
pub fn add(a: &Vector<f32, S>, b: &Vector<f32, S>) -> Vector<f32, S> { a + b }
pub fn internal_add(a: &Vector<f32, S>, b: &Vector<f32, S>) -> Vector<f32, S> {
	Vector::build_with_fn(|i| {
		let a: &f32 = a.i(i);
		let b: &f32 = b.i(i);
		a + b
	})
}
