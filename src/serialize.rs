use crate::Vector;
use serde::ser::{Serialize, SerializeTuple, Serializer};

impl<T, const N: usize> Serialize for Vector<T, N>
where
	T: Serialize,
{
	fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
		let mut el = s.serialize_tuple(N)?;
		for i in self {
			el.serialize_element(i)?;
		}
		el.end()
	}
}

use core::fmt;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};

impl<'de, T, const N: usize> Deserialize<'de> for Vector<T, N>
where
	T: Deserialize<'de>,
{
	fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
		let visitor = ElementVisitor::<T, N>(Default::default());
		d.deserialize_tuple(N, visitor)
	}
}

struct ElementVisitor<T, const N: usize>(core::marker::PhantomData<T>);

impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for ElementVisitor<T, N> {
	type Value = Vector<T, N>;
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "a sequence of {} elements", N)
	}
	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		//fixme: fail softly/by returning a Result
		use crate::types::Stupidity;
		Ok(Vector::<T, N>::build_with_fn(|_| {
			seq.next_element().unwrap().unwrap()
		}))
	}
}

#[test]
fn ser_de_test() {
	use crate::Matrix;
	use core::array::FixedSizeArray;
	use rand::{thread_rng, Rng};
	let mut rng = thread_rng();

	let matrix: Matrix<u32, 20, 40> = rng.gen();

	let mut buf: [u8; 20 * 40 * 4] = [0; 20 * 40 * 4];
	bincode::serialize_into(buf.as_mut_slice(), &matrix).unwrap();

	let decoded = bincode::deserialize(&buf[..]).unwrap();
	assert_eq!(matrix, decoded);
}
