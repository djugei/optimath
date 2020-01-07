<!-- cargo-sync-readme start -->

# Optimath

A Linear Algebra library that uses const generics to be no_std and specialization to enable SIMD*.

*simd blocked on compiler bug, autovectorization works well though.

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

The whole library is built around just one type, [Vector<T, N>](Vector) representing a Vector of N
elements of type T.

In case T supports some math operation like addition (implements the Add trait) the Vector too
supports that as an element-wise operation. As such a Vector<Vector<T: Add, N>, M> also
supports addition, due to Vector<T: Add, N> being a type that implements Add.

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
those features. The [insights] module contains some of that.

It is also meant to explore the design space of Linear Algebra libraries that utilize those
features. As such it may serve as inspiration for how bigger linalg libraries might adopt
them.

## Changelog (and future)

### 0.1.0
* A Vector type that can do element-wise maths
* Basic linear algebra operations
* A sturdy design for future improvements

### 0.2.0 (current)
* serde support
* rand support

### 0.3.0 (current-dev)
* moved more iterating over to ConstIterator

### 0.3.0
* [ ] rearchitecture a bit so Vectors are generic over containers
* [ ] strided iteration over matrices
* [ ] windows-function

### 0.4.0
* [ ] working SIMD on Vectors (blocked on rust compiler bug(s), but auto-vectorization works
super well)
* [ ] additional operations on Vectors and Matrixes (taking feature requests!)


### 0.5.0
* [ ] interaction with dynamically sized vectors
    * [ ] widows-function on dynamically sized vectors

### 0.6.0
* [ ] multi-threading for really large workloads

### 0.7.0
* [ ] full specialized SIMD for sse, avx and avx512
* [ ] full SIMD between Vectors, dynamic Vectors and vector views

### 0.8.0
* [ ] a BLAS compatible interface, including a C-interface. Probably in a different crate based
on this
* [ ] have 2 additional contributors :) come join the fun and headache about weird compiler bugs
and pointer offset calculations

### 1.0.0
* [ ] been used/tested in other peoples crates and considered usable


## Ideas section

Currently the crate is built up from vectors, could instead be built "down" from dimensions
see the (private) dimensional module for a sketch of that. its currently blocked on rust not
being able to actually use any calculations for const-generic array sizes.
positive: enable easier iteration/strided iteration as that would just be plain pointer maths.
negative: harder/impossible to express explicit simd.


Automatically build Vectors to be ready for simd and/or multiprocessing. also blocked on
the same rust feature of calculated array sizes. see the (private) layout module for a preview.
im not sure this is necessary though, seeing that with the sizes know at compile time rust
generates very good simd and unrolls.
positive: perfect simd every time on every platform. negative: higher workload, need to take
care for every operation and every platform. negative: transposed and strided iteration gets
harder

For interoperability it would be nice to express things either being sized or unsized.
especially for dimensions like matrix multiplication, U x S(3) * S(3) x U = U x U could be a
common case to self multiply a list with unknown number of entries but known number of features
(this is probably also blocked on the same rust bug, but i did not test yet)

Instead of instantly doing the calculations, provide an Iterator-like interface that builds a
calculation and then runs it element by element as much as possible (matrix multiplication
being a challenge ofc). that would allow for better cache-locality/less memory access.
important, as after simd memory bandwidth seems to be the main bottleneck.

<!-- cargo-sync-readme end -->

# Contributing
Please symlink the hooks to your local .git/hooks/ directory to run some automatic checks before committing.

    ln -s ../../hooks/pre-commit .git/hooks/

Please install rustfmt and cargo-sync-readme so these checks can be run.

    rustup component add rustfmt
    cargo install cargo-sync-readme

Please execute `cargo-sync-readme` when you change the top-level-documentation.
Please run `cargo fmt` whenever you change code. If possible configure your editor to do so for you.
