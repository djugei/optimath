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

use core::{fmt, mem::MaybeUninit};
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};

impl<'de, T, const N: usize> Deserialize<'de> for Vector<T, N>
where
	T: Deserialize<'de>,
{
	fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
		let visitor = ElementVisitor::<T, N> {
			elements: MaybeUninit::uninit(),
		};
		let inner = d.deserialize_tuple(N, visitor)?;
		Ok(Self { inner })
	}
}

struct ElementVisitor<T, const N: usize> {
	elements: MaybeUninit<[T; N]>,
}

impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for ElementVisitor<T, N> {
	type Value = [T; N];
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "a sequence of {} elements", N)
	}
	fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let el_pointer = self.elements.as_mut_ptr() as *mut T;
		for offset in 0..N {
			if let Some(element) = seq.next_element()? {
				// this can't overshoot cause offset is 0..N
				unsafe { el_pointer.add(offset).write(element) };
			} else {
				return Err(serde::de::Error::custom("not enough elements"));
			}
		}
		// when we arrive here, N elements are guaranteed to be initalized
		let elements = unsafe { self.elements.assume_init() };
		Ok(elements)
	}
}

#[test]
fn ser_de_test() {
	use crate::Matrix;
	use core::array::FixedSizeArray;

	let matrix: Matrix<u32, 20, 40> = (0..40).map(|r| (0..20).map(|c| r * c).collect()).collect();

	let mut buf: [u8; 20 * 40 * 4] = [0; 20 * 40 * 4];
	bincode::serialize_into(buf.as_mut_slice(), &matrix).unwrap();

	let decoded = bincode::deserialize(&buf[..]).unwrap();
	assert_eq!(matrix, decoded);
}
