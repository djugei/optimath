use crate::types::Vector;
use core::ops::*;

// reference operations
// need to have the Output = T on the Add for &T, otherwise you get infinite recursion
macro_rules! impl_op {
	( $op:tt, $fn:ident, $basetype:ty, $( $generics:tt),  *; $( $cons:tt $constype:ty ), * ) => {
		impl<'a, 'b, $( $generics), *, $( const $cons : $constype), *> $op <&'b $basetype> for &'a $basetype
		where
			&'a T: $op<&'b T, Output = T>,
		{
			type Output = $basetype;
			default fn $fn(self, other: &'b $basetype) -> $basetype {
				self.into_iter()
					.zip(other)
					.map(|(s, o)| $op::$fn(s, o))
					.collect()
			}
		}
	};
}

macro_rules! maths {
	( $basetype:ty, $( $generics:tt),  *; $( const $cons:tt : $constype:ty ), * ) => {
            impl_op!(Add, add, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_op!(Sub, sub, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_op!(Mul, mul, $basetype, $( $generics),  *; $( $cons $constype ), * );
            impl_op!(Div, div, $basetype, $( $generics),  *; $( $cons $constype ), * );
        }
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

maths!(Vector<T, N>, T; const N: usize);
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
	let a: Vector<f32, TESTLEN> = (0..TESTLEN).map(|x| x as f32).collect();
	let b: Vector<f32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as f32).collect();

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
	let a: Vector<f32, TESTLEN> = (0..TESTLEN).map(|x| x as f32).collect();
	let b: Vector<f32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as f32).collect();

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
