use core::ops::Add;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use optimath::Vector;

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
	let a: Vector<f32, TESTLEN> = (0..TESTLEN).map(|x| x as f32).collect();
	let b: Vector<f32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as f32).collect();

	let mut group = c.benchmark_group("addition");
	group.warm_up_time(core::time::Duration::from_millis(200));
	group.measurement_time(core::time::Duration::from_secs(2));
	group.sample_size(250);

	group.bench_function("f32 simd", |bench| bench.iter(|| black_box(&a + &b)));

	// this is currently rather slow, since for iteration the whole array is ptr.read()
	// cause transmute doesn't work with const generics yet
	group.bench_function("f32 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});

	let a: Vector<Ff32, TESTLEN> = (0..TESTLEN).map(|x| Ff32(x as f32)).collect();
	let b: Vector<Ff32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| Ff32(x as f32)).collect();

	group.bench_function("f32 scalar", |bench| bench.iter(|| black_box(&a + &b)));
}

pub fn mul(c: &mut Criterion) {
	let mut group = c.benchmark_group("sizes");
	group.warm_up_time(core::time::Duration::from_millis(200));
	group.measurement_time(core::time::Duration::from_secs(1));
	group.sample_size(500);

	let a: Vector<u8, TESTLEN> = (0..TESTLEN).map(|x| x as u8).collect();
	let b: Vector<u8, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as u8).collect();
	group.bench_function("u8", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u8 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});

	let a: Vector<u16, TESTLEN> = (0..TESTLEN).map(|x| x as u16).collect();
	let b: Vector<u16, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as u16).collect();
	group.bench_function("u16", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u16 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});

	let a: Vector<u32, TESTLEN> = (0..TESTLEN).map(|x| x as u32).collect();
	let b: Vector<u32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as u32).collect();
	group.bench_function("u32", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u32 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});

	let a: Vector<u64, TESTLEN> = (0..TESTLEN).map(|x| x as u64).collect();
	let b: Vector<u64, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as u64).collect();
	group.bench_function("u64", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u64 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});

	let a: Vector<u128, TESTLEN> = (0..TESTLEN).map(|x| x as u128).collect();
	let b: Vector<u128, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as u128).collect();
	group.bench_function("u128", |bench| bench.iter(|| black_box(&a * &b)));
	group.bench_function("u128 noabstract", |bench| {
		bench.iter(|| {
			for (a, b) in a.into_iter().zip(b) {
				black_box(a + b);
			}
		})
	});
}

criterion_group!(sse3, add, mul);
criterion_main!(sse3);
