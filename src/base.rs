use crate::{consts::ConstIndex, types::Vector};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// reference operations
// need to have the Output = T on the Add for &T, otherwise you get infinite recursion

// im not good with macro hygene, but foring the size to be called N and the type to be called T
// seems wrong to me and like its breaking hygene..
macro_rules! impl_op {
	( $op:tt, $fn:ident, $basetype:ty, $( $generics:tt),  *; $( $cons:tt $constype:ty ), * ) => {
		impl<'a, 'b, $( $generics), *, B: 'b, $( const $cons : $constype), *> $op <B> for $basetype
                where
                        &'a T: $op<&'b T, Output = T>,
                        B: ConstIndex<&'b T, N> + Copy + Clone,
                        T: 'a + 'b,
                {
                        type Output = Vector<T, N>;
                        default fn $fn(self, other: B) -> Vector<T, N> {
                                self.into_iter()
                                        .enumerate()
                                        .map(|(i, s)| $op::$fn(s, other.i(i)))
                                        .collect()
                        }
                }
	};
}

macro_rules! maths {
	( $basetype:ty, $( $generics:tt), *; $( const $cons:tt : $constype:ty ), * ) => {
            impl_op!(Add, add, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_op!(Sub, sub, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_op!(Mul, mul, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_op!(Div, div, $basetype, $( $generics),  *; $( $cons $constype ), * );
	};
}

// assigning operations
macro_rules! impl_assign_op {
	( $op:tt, $fn:ident, $basetype:ty, $( $generics:tt),  *; $( $cons:tt $constype:ty ), * ) => {
		impl<'a, $( $generics), *, $( const $cons : $constype), *> $op <&'a $basetype> for $basetype
		where
			T: $op<&'a T>,
		{
			fn $fn(&mut self, other: &'a $basetype) {
				let iter = self.inner.iter_mut().zip(other);
				for (s, o) in iter {
					$op::$fn(s, o);
				}
			}
		}

		impl<'a, 'b, $( $generics), *, B: 'b, $( const $cons : $constype), *> $op <B> for $basetype
                where
                        T: $op<&'b T>,
                        B: ConstIndex<&'b T, N> + Copy + Clone,
                        T: 'a + 'b,
                {
                        default fn $fn(&mut self, other: B) {
                            use crate::consts::ConstIteratorMut;
				let iter  = ConstIteratorMut::from(self);
				for (i, s) in iter.enumerate() {
					$op::$fn(s, other.i(i));
				}
                        }
                }
	};
}

macro_rules! assign_maths {
	( $basetype:ty, $( $generics:tt),  *; $( const $cons:tt : $constype:ty ), * ) => {
            impl_assign_op!(AddAssign, add_assign, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_assign_op!(SubAssign, sub_assign, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_assign_op!(MulAssign, mul_assign, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_assign_op!(DivAssign, div_assign, $basetype, $( $generics),  *; $( $cons $constype ), * );
        }
}

maths!(&'a Vector<T, N>, T; const N: usize);

use crate::VectorView;
maths!(VectorView<'a, T, N, M>, T; const N: usize, const M: usize);

assign_maths!(Vector<T, N>, T; const N: usize);

#[cfg(test)]
pub(crate) const TESTLEN: usize = 777usize;

#[test]
fn default_is_default() {
	let m = Vector::<f32, TESTLEN>::default();
	for i in 0..TESTLEN {
		assert_eq!(m.inner[i], f32::default());
	}
}

#[test]
fn operations() {
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();
	let a: Vector<f32, TESTLEN> = rng.gen();
	let b: Vector<f32, TESTLEN> = rng.gen();

	let add = &a + &b;
	let sub = &a - &b;
	let mul = &a * &b;
	let div = &a / &b;

	for i in 0..TESTLEN {
		assert_eq!(a.inner[i] + b.inner[i], add.inner[i]);
		assert_eq!(a.inner[i] - b.inner[i], sub.inner[i]);
		assert_eq!(a.inner[i] * b.inner[i], mul.inner[i]);
		assert_eq!(a.inner[i] / b.inner[i], div.inner[i]);
	}
}

#[test]
fn assignment_operations() {
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();
	let a: Vector<f32, TESTLEN> = rng.gen();
	let b: Vector<f32, TESTLEN> = rng.gen();

	let mut add = a.clone();
	add += &b;

	let mut sub = a.clone();
	sub -= &b;

	let mut mul = a.clone();
	mul *= &b;

	let mut div = a.clone();
	div /= &b;

	for i in 0..TESTLEN {
		assert_eq!(a.inner[i] + b.inner[i], add.inner[i]);
		assert_eq!(a.inner[i] - b.inner[i], sub.inner[i]);
		assert_eq!(a.inner[i] * b.inner[i], mul.inner[i]);
		assert_eq!(a.inner[i] / b.inner[i], div.inner[i]);
	}
}
