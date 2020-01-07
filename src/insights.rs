//! Some insight about simd, autovectorization, const generics and specialization gained from building this library
//!
//! i used a lot of experimental parts of rust. this is supposed to give some feedback on how
//! usable i feel they are and where issues remain. This library is very Type Astronaut-y so this
//! feedback is for the bleeding edge and will not represent the average usecase.
//!
//! # SIMD
//! simd is a pain in the butt to deal with. This is true in general and specifically for rust.
//! Luckily autovectorization is quite reliable if you try to play nicely with the compiler
//!
//! packed_simd has more #[doc(hidden)] than i have fingers, especially the core defining trait,
//! SimdArray is private, which is annoying if you want to build a library thats generic over simd.
//!
//! simd is very platform dependent, so you have to choose to either just ask people to
//! "-C target-cpu=native" or you need to provide a way to at runtime switch between
//! implementations.
//!
//! rn you can't build your structs to suit your simd-needs bc compiler-bug/unfinishedness
//!
//! # Autovectorization
//! Autovetcorization actually works quite well, at least in my usecase. the problem is
//! reliability. i can't assure a loop actually gets vectorized. it is kinda similar to tail call
//! optimization. a solution would be to have an attribute that forces optimization or fails the
//! compile.
//!
//! # Const Generics
//! Seem quite cool to me, but severely limited by the inability to build vectors of sizes
//! calculated at compile time. once you can do that some really nice stuff becomes possible.
//!
//! Limitations are that there is no dedicated place to do const calculations on structs as you
//! can't have associated constants on them. that will especially be annoying if const generics
//! need to come from the same "place/expression" to be considered equal. a workaround is to
//! have a wrapper type that does the calculations and an inner type that stores the results.
//! Right now that leads to the inner struct not being buildable, but i think thats the same bug
//! that also stops calculations in array sizes.
//!
//! i feel like there is some higher level concept unexpressed, a type whose size is known, but not
//! nescesarily at compile time, kinda similar to TrustedLen.
//!
//! also, slightly related, its not possible to build a completely different struct with different
//! fields based on generics or const generics. maybe thats a good thing though.
//!
//! # specialization
//! really cool concept, have not had too many issues except some weird type inference bugs on
//! (imo) unrelated places, but im not 100% sure those are due to specialization, could also be one
//! of the other 10 nightly features.
//!
//! whats a bit weird is having a trait that has associated types and methods taking or returning
//! that associated type, both being default.
//! because when writing a default implementation you can't assume anything about the type since it
//! could be overridden independently from the function. this is not clear from the error messages
//! though. a solution here is to constrain the associated type by traits in the trait definition
//! and only rely on trait methods. this does not work for return types though.
//!
//! also specialization feels suspiciously like inheritance
//!
//! # Generic Associated Types (GAT)
//! there are multiple places in the library where i think GATs would have been usefull, mainly to
//! stop combinatorial explosion of diferent vector-types and Add/Sub/Mul between them. The
//! compiler would then on-demand be able to instanciate them into the code instead of me having to
//! macro them.
//!
//! so im looking forward to them being implemented so i can play with them a bit
//!
//! # The Index trait
//! sucks hard, cause it requires you to treturn an actual reference to something stored in the
//! type, can't use it to on the fly generate a view (not 100% true as there is the fat pointer
//! trick but thats not really... documented behaviour). had the Iterator trait made the same
//! decision rust would be in a horrible state right now.
