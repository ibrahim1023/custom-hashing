use criterion::{ black_box, Criterion };

pub fn benchmark_custom_hash(c: &mut Criterion) {
    let input = "Benchmarking custom hash function";

    c.bench_function("custom_hash1", |b|
        b.iter(|| black_box(crate::core::custom_hash1::custom_hash1(input)))
    );
    c.bench_function("custom_hash2", |b|
        b.iter(|| black_box(crate::core::custom_hash2::custom_hash2(input)))
    );
}
