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

const TESTLEN: usize = 777;

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

criterion_group!(sse3, add);
criterion_main!(sse3);
