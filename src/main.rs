use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

// Original function
fn quantize_15_original(lut: &[f32]) -> (f32, f32, Vec<u8>) {
    let min = lut.iter().copied().fold(f32::INFINITY, f32::min);
    let max = lut.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let k = (max - min).max(0.0) / 15.0;
    let b = min;
    (k, b, lut.iter().map(|&y| ((y - b) / k) as u8).collect())
}

// Optimized function
fn quantize_15_optimized(lut: &[f32]) -> (f32, f32, Vec<u8>) {
    if lut.is_empty() {
        return (0.0, 0.0, Vec::new());
    }

    let (min, max) = lut.iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), &val| {
            (min.min(val), max.max(val))
        });

    let k = (max - min).max(0.0) / 15.0;
    let b = min;

    let quantized = if k == 0.0 {
        vec![0; lut.len()]
    } else {
        let scale = 15.0 / (max - min);
        lut.iter()
            .map(|&y| ((y - min) * scale) as u8)
            .collect()
    };

    (k, b, quantized)
}

fn generate_random_lut(size: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(-1000.0..1000.0)).collect()
}

fn benchmark_quantize_15(c: &mut Criterion) {
    let small_lut = generate_random_lut(100);
    let medium_lut = generate_random_lut(10_000);
    let large_lut = generate_random_lut(1_000_000);

    let mut group = c.benchmark_group("quantize_15");

    group.bench_function("original_small", |b| b.iter(|| quantize_15_original(black_box(&small_lut))));
    group.bench_function("optimized_small", |b| b.iter(|| quantize_15_optimized(black_box(&small_lut))));

    group.bench_function("original_medium", |b| b.iter(|| quantize_15_original(black_box(&medium_lut))));
    group.bench_function("optimized_medium", |b| b.iter(|| quantize_15_optimized(black_box(&medium_lut))));

    group.bench_function("original_large", |b| b.iter(|| quantize_15_original(black_box(&large_lut))));
    group.bench_function("optimized_large", |b| b.iter(|| quantize_15_optimized(black_box(&large_lut))));

    group.finish();
}

criterion_group!(benches, benchmark_quantize_15);
criterion_main!(benches);