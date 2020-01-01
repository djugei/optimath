#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(maybe_uninit_extra)]
//#![feature(avx512_target_feature)]
#![allow(dead_code)]
#![feature(associated_type_defaults)]

mod advanced;
mod base;
mod layout;
mod simd_impl;
mod types;
mod view;

#[cfg(all(
	target_arch = "x86_64",
	target_feature = "sse",
	not(target_feature = "avx")
))]
mod sse;

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
mod avx;

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
use avx as simd;

#[cfg(all(
	target_arch = "x86_64",
	target_feature = "sse",
	not(target_feature = "avx")
))]
use sse as simd;


pub use types::{Matrix, Vector};
pub use view::{TransposedMatrixView, VectorView};
// add a type like StaticSizedIterator to make reasoning about dimensions easier
