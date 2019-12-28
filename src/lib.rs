#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(maybe_uninit_extra)]
//#![feature(avx512_target_feature)]
//#![feature(sse4a_target_feature)]

mod advanced;
mod base;
mod view;
mod types;

pub use types::{FullMath, Matrix, Vector};

