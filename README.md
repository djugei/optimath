<!-- cargo-sync-readme start -->

# Optimath

A Linear Algebra library that uses const generics to be no_std and specialization to enable SIMD.

## Examples

### Element-wise addition

    use optimath::Vector;
    let a: Vector<i32, 2000> = (0..2000).collect();
    let b: Vector<i32, 2000> = (0..2000).collect();

    // operations are only defined on references to Vectors
    let c = &a + &b;

    let r: Vector<i32, 2000> = (0..2000).map(|s| s+s).collect();

    assert_eq!(c, r);

### Matrix multiplication

    use optimath::Matrix;
    let a: Matrix<f32, 2, 3> = Default::default();
    let b: Matrix<f32, 3, 4> = Default::default();

    // matrix size is checked at compile time!
    let c: Matrix<f32, 2, 4> = a.matrix_multiply(&b);

## Design

The whole library is built around just one type, Vector<T, N> representing a Vector of N
elements of type T.

In case T supports some math operation like addition (implements the Add trait) the Vector too
supports that as an element-wise operation. As such a Vector<Vector<T: Add, N>, M> also
supports addition, due to Vector<T: Add, N> being a type that supports Add.

Matrices and Tensors are therefore just Vectors within Vectors (within Vectors)

### no_std

const generics are used to enable Vectors to contain any (fixed) number of elements and
therefore not require allocation on the heap.

### SIMD

Vectors provide generic math operations for any T that implements that operation.
specialization is used to provide optimized implementations for specific T, like for example
floats and integers.

At this moment SIMD support is disabled while we wait for rustc to fix some ICE :).

## Goals

Besides being hopefully useful as a library it is also an exploration of rusts newer advanced
type system features. It is therefore an explicit goal to provide feedback to the developers of
those features.

It is also meant to explore the design space of Linear Algebra libraries that utilize those
features. As such it may serve as inspiration for how bigger linalg libraries might addopt
them.

## Changelog (and future)

### 0.1.0 (current)
* A Vector type that can do element-wise maths
* Basic linear algebra operations
* A sturdy design for future improvements

### 0.2.0
* [ ] working SIMD on Vectors (blocked on rust compiler bug(s))
* [ ] additional operations on Vectors and Matrixes (taking feature requests!)

### 0.3.0
* [ ] strided iteration over matrices
* [ ] windows-function

### 0.4.0
* [ ] interaction with dynamically sized vectors
    * [ ] widows-function on dynamically sized vectors

### 0.5.0
* [ ] multi-threading for really large workloads

### 0.6.0
* [ ] full specialized SIMD for sse, avx and avx512
* [ ] full SIMD between Vectors, dynamic Vectors and vector views

### 0.7.0
* [ ] a BLAS compatible interface, including a C-interface. Probably in a different crate based
on this
* [ ] have 2 contributors :) come join the fun and headache about weird compiler bugs and
pointer offset calculations

### 1.0.0
* [ ] been used/tested in other peoples crates and considered usable

<!-- cargo-sync-readme end -->

# Contributing
Please symlink the hooks to your local .git/hooks/ directory to run some automatic checks before committing.

    ln -s ../../hooks/pre-commit .git/hooks/

Please install rustfmt and cargo-sync-readme so these checks can be run.

    rustup component add rustfmt
    cargo install cargo-sync-readme

Please execute `cargo-sync-readme` when you change the top-level-documentation.
Please run `cargo fmt` whenever you change code. If possible configure your editor to do so for you.
