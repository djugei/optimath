use crate::{ConstIndex, Stupidity, Vector};

const S: usize = 250;
pub fn add(a: &Vector<f32, S>, b: &Vector<f32, S>) -> Vector<f32, S> { a + b }
pub fn internal_add(a: &Vector<f32, S>, b: &Vector<f32, S>) -> Vector<f32, S> {
	Vector::build_with_fn(|i| a.i(i) + b.i(i))
}
