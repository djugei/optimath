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

const TESTLEN: usize = 55;

pub fn add(c: &mut Criterion) {
	let a: Vector<f32, TESTLEN> = (0..TESTLEN).map(|x| x as f32).collect();
	let b: Vector<f32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as f32).collect();

	let mut group = c.benchmark_group("addition");
	group.warm_up_time(core::time::Duration::from_millis(200));
	group.measurement_time(core::time::Duration::from_secs(1));
	group.sample_size(2000);

	group.bench_function("f32 simd", |bench| {
		bench.iter(|| black_box(&black_box(a) + &black_box(b)))
	});

	let a: Vector<Ff32, TESTLEN> = (0..TESTLEN).map(|x| Ff32(x as f32)).collect();
	let b: Vector<Ff32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| Ff32(x as f32)).collect();

	group.bench_function("f32 scalar", |bench| {
		bench.iter(|| black_box(&black_box(a) + &black_box(b)))
	});
}

criterion_group!(sse3, add);
criterion_main!(sse3);
