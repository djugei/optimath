use core::mem::MaybeUninit;
use core::iter::FromIterator;

/// size_of::<T>() * num == size_of::<Repr>()
pub trait SimdRepr: Sized {
	const NUM: usize;
	type Repr;
    //todo: automatically build this type, dunnow how tho
    //also there is a compiler bug with calculations in array sizes specifically
    type Unpacked = [Self; 1];
}

// need to find a way to pack&unpack data into/from the iter
// default trait does not rly work it seems :(
//
// maybe i can do a default impl with 1-1 "casts" for normal types
// and specific stuff for stuff with simd impls? probably
pub trait SimdPack: SimdRepr {
    fn pack(un: Self::Unpacked) -> Self::Repr;
//    fn pack_ref(&self) -> &<Self as SimdRepr>::Repr;

//    fn unpack(pack: <Self as SimdRepr>::Repr) -> Self;
//    fn unpack_ref(pack: &<Self as SimdRepr>::Repr) -> Self;
}

impl<T> SimdRepr for T {
	default type Repr = T;
	default const NUM: usize = 1;
    default type Unpacked = [Self; 1];
}

pub struct Vecc<T, const N: usize> {
    i: Inner<
        T,
        {N / <T as SimdRepr>::NUM},
        {N / <T as SimdRepr>::NUM}, 
        {(N / <T as SimdRepr>::NUM) * N}, 
    >,
}

pub(crate) struct Inner<T, const CHUNKS: usize, const LEFTOVER: usize, const IN_BASE: usize> {
    base: [[<T as SimdRepr>::Repr; <T as SimdRepr>::NUM]; CHUNKS],
    spill: [T; LEFTOVER],
}

impl<T, const CHUNKS: usize, const LEFTOVER: usize, const IN_BASE: usize> FromIterator<T> for
    Inner<T, CHUNKS, LEFTOVER, IN_BASE> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut iter = iter.into_iter();

            let mut base  = MaybeUninit::uninit();
            let mut spill = MaybeUninit::uninit();

            let base_ptr = &mut base as *mut _ as *mut T;
            for offset in 0..Self::IN_BASE {
                if let Some(element) = iter.next() {
                    unsafe {
                        base_ptr.add(offset).write(element);
                    }
                } else {
                    panic!("not enough elements");
                }
            }
            let base = unsafe { base.assume_init() };

            let spill_ptr = &mut spill as *mut _ as *mut T;
            for offset in 0..Self::IN_SPILL {
                if let Some(element) = iter.next() {
                    unsafe {
                        spill_ptr.add(offset).write(element);
                    }
                } else {
                    panic!("not enough elements");
                }
            }
            let spill = unsafe { spill.assume_init() };

            iter.next().map(|_| panic!("too many elements"));

            Inner { base, spill }
        }
}

/*pub struct Wrap<T: Sized, const N: usize> {
	base: Base<T, { N / <T as SimdRepr>::NUM }>,
	spill: Spill<T, { N % <T as SimdRepr>::NUM }>,
}

impl<T, const N: usize> Wrap<T, N> {
    const BASE_CHUNKS: usize = (N / <T as SimdRepr>::NUM);
    const IN_BASE: usize = Self::BASE_CHUNKS*N;
	const IN_SPILL: usize = N % <T as SimdRepr>::NUM;
}

default impl<T: Sized, const N: usize> FromIterator<T> for Wrap<T, N> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // fixme: do from_iter seperately for base and spill
	}
}

#[repr(transparent)]
pub struct Base<T: Sized, const N: usize> {
	inner: [<T as SimdRepr>::Repr; N],
}

#[repr(transparent)]
pub struct Spill<T: Sized, const N: usize> {
	content: [T; N],
}
*/
