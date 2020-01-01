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

default impl<T, const N: usize> FromIterator<T> for Vecc<T, N> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            //compiler bug :((((
            let i = Inner::from_iter(iter);
            Self { i }
        }
}

pub(crate) struct Inner<T, const CHUNKS: usize, const SPILL: usize, const IN_BASE: usize> {
    base: [[<T as SimdRepr>::Repr; <T as SimdRepr>::NUM]; CHUNKS],
    spill: [T; SPILL],
}

default impl<T, const CHUNKS: usize, const SPILL: usize, const IN_BASE: usize> FromIterator<T> for
    Inner<T, CHUNKS, SPILL, IN_BASE> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut iter = iter.into_iter();

            let mut base  = MaybeUninit::uninit();
            let mut spill = MaybeUninit::uninit();

            let base_ptr = &mut base as *mut _ as *mut T;
            for offset in 0..IN_BASE {
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
            for offset in 0..SPILL {
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

#[test]
fn base_build() {
    let a: Vecc<usize, 77> = (0..77).map(|a| a as f32).collect();
}

use core::arch::x86_64::{__m128, _mm_loadu_ps};
use crate::layout::SimdRepr;

impl SimdRepr for f32 {
	const NUM: usize = 4;
	type Repr = __m128;
    type Unpacked = [Self; 4];
}

default impl<const CHUNKS: usize, const SPILL: usize, const IN_BASE: usize> FromIterator<T> for
    Inner<T, CHUNKS, SPILL, IN_BASE> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut iter = iter.into_iter().chunks();

            let mut base  = MaybeUninit::uninit();
            let mut spill = MaybeUninit::uninit();

            let base_ptr = &mut base as *mut _ as *mut __m128;

            for offset in 0..IN_BASE {
                if let Some(chunk) = iter.next() {
                    if chunk.len() < <Self as SimdRepr>::NUM {
                        panic!("not enough elements in chunk")
                    }
                    unsafe {
                        let e = _mm_loadu_ps(chunk.as_ptr());
                        base_ptr.add(offset).write(e);
                    }
                } else {
                    panic!("not enough elements");
                }
            }
            let base = unsafe { base.assume_init() };

            let spill_ptr = &mut spill as *mut _ as *mut T;
            for offset in 0..SPILL {
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

#[test]
fn f32_build() {
    let a: Vecc<f32, 77> = (0..77).map(|a| a as f32).collect();
}
