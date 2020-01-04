use core::ops::Add;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use optimath::Vector;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
#[repr(transparent)]
struct Ff32(f32);

impl Add for Ff32 {
	type Output = Ff32;
	fn add(self, other: Self) -> Self { Ff32(self.0 + other.0) }
}

impl<'a, 'b> Add<&'b Ff32> for &'a Ff32 {
	type Output = Ff32;
	fn add(self, other: &'b Ff32) -> Ff32 { Ff32(self.0 + other.0) }
}

const TESTLEN: usize = 250;

pub fn add(c: &mut Criterion) {
	let mut rng = thread_rng();
	let a: Vector<f32, TESTLEN> = rng.gen();
	let b: Vector<f32, TESTLEN> = rng.gen();

	// there seem to be two "modes" hit on optimization
	// currently "f32 simd" hits the good one
	// and "f32 noabstract" and "f32 scalar" hit the bad one
	// good one takes 180, bad one 320
	// "f32 inline" takes the middle ground at 250 even though it should be identical to
	// "f32 simd"
	// can't inspect the asm of this specific benchmark though as the compiler just locks up
	// and eats 30 GB ram.
	// asm of benching::add and benching::internal_add seem identical and very well vectorized
	// so im assuming this a benchmarking problem
	let mut group = c.benchmark_group("addition");
	group.warm_up_time(core::time::Duration::from_millis(200));
	group.measurement_time(core::time::Duration::from_secs(2));
	group.sample_size(250);

	group.bench_function("f32 inline", |bench| bench.iter(|| &a + &b));
	group.bench_function("f32 simd", |bench| {
		bench.iter(|| optimath::benching::add(&a, &b))
	});

	group.bench_function("f32 noabstract", |bench| {
		bench.iter(|| black_box(optimath::benching::internal_add(&a, &b)))
	});

	let a: Vector<f32, TESTLEN> = rng.gen();
	let b: Vector<f32, TESTLEN> = rng.gen();
	let a: Vector<Ff32, TESTLEN> = a.into_iter().map(|f: f32| Ff32(f)).collect();
	let b: Vector<Ff32, TESTLEN> = b.into_iter().map(|f: f32| Ff32(f)).collect();

	group.bench_function("f32 scalar", |bench| bench.iter(|| black_box(&a + &b)));
}

pub fn mul(c: &mut Criterion) {
	let mut rng = thread_rng();
	let mut group = c.benchmark_group("sizes");
	group.warm_up_time(core::time::Duration::from_millis(200));
	group.measurement_time(core::time::Duration::from_secs(1));
	group.sample_size(500);

	let a: Vector<u8, TESTLEN> = rng.gen();
	let b: Vector<u8, TESTLEN> = rng.gen();
	group.bench_function("u8", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u8 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a * b);
			}
		})
	});

	let a: Vector<u16, TESTLEN> = rng.gen();
	let b: Vector<u16, TESTLEN> = rng.gen();
	group.bench_function("u16", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u16 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a * b);
			}
		})
	});

	let a: Vector<u32, TESTLEN> = rng.gen();
	let b: Vector<u32, TESTLEN> = rng.gen();
	group.bench_function("u32", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u32 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});

	let a: Vector<u64, TESTLEN> = rng.gen();
	let b: Vector<u64, TESTLEN> = rng.gen();
	group.bench_function("u64", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u64 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});

	let a: Vector<u128, TESTLEN> = rng.gen();
	let b: Vector<u128, TESTLEN> = rng.gen();
	group.bench_function("u128", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u128 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});
}

const BIG: usize = 40_001;

pub fn create(c: &mut Criterion) {
	use core::mem::MaybeUninit;
	let mut group = c.benchmark_group("create");
	group.warm_up_time(core::time::Duration::from_millis(200));
	group.measurement_time(core::time::Duration::from_secs(2));
	group.sample_size(250);

	group.bench_function("uninit", |bench| {
		bench.iter(|| {
			black_box({
				let b: MaybeUninit<[f32; BIG]> = MaybeUninit::uninit();
				b
			})
		})
	});

	group.bench_function("write", |bench| {
		bench.iter(|| {
			black_box({
				let mut b: MaybeUninit<[f32; BIG]> = MaybeUninit::uninit();
				let b_ptr = b.as_mut_ptr() as *mut f32;
				for i in 0..BIG {
					unsafe {
						b_ptr.add(i).write(0.);
					}
				}
				unsafe { b.assume_init() }
			})
		})
	});
}

criterion_group!(sse3, add, mul, create);
criterion_main!(sse3);
