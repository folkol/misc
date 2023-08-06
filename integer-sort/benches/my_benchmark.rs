use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;

use integer_sort::{integer_sort, sort};

fn gen_input(p: usize, i: u64) -> Vec<u64> {
    let rng = StdRng::seed_from_u64(1337);
    rng.sample_iter(Standard)
        .take(p)
        .map(|x: u64| x % i)
        .collect()
}

// TODO: Parameterize

fn sort_small_few(c: &mut Criterion) {
    let range = 10;
    c.bench_function("sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(1000, range) },
            |v| sort(v),
            BatchSize::SmallInput,
        )
    });
}

fn integer_sort_small_few(c: &mut Criterion) {
    let range = 10;
    c.bench_function("integer-sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(1000, range) },
            |v| integer_sort(v, 0, range as i32),
            BatchSize::SmallInput,
        )
    });
}
fn sort_large_few(c: &mut Criterion) {
    let range = 10;
    c.bench_function("sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(10_000_000, range) },
            |v| sort(v),
            BatchSize::LargeInput,
        )
    });
}

fn integer_sort_large_few(c: &mut Criterion) {
    let range = 10;
    c.bench_function("integer-sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(10_000_000, range) },
            |v| integer_sort(v, 0, range as i32),
            BatchSize::LargeInput,
        )
    });
}

fn sort_small_many(c: &mut Criterion) {
    let range = 1000000;
    c.bench_function("sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(1000, range) },
            |v| sort(v),
            BatchSize::SmallInput,
        )
    });
}

fn integer_sort_small_many(c: &mut Criterion) {
    let range = 1000000;
    c.bench_function("integer-sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(1000, range) },
            |v| integer_sort(v, 0, range as i32),
            BatchSize::SmallInput,
        )
    });
}
fn sort_large_many(c: &mut Criterion) {
    let range = 1000000;
    c.bench_function("sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(10_000_000, range) },
            |v| sort(v),
            BatchSize::LargeInput,
        )
    });
}

fn integer_sort_large_many(c: &mut Criterion) {
    let range = 1000000;
    c.bench_function("integer-sort", |b| {
        b.iter_batched(
            || -> Vec<_> { gen_input(10_000_000, range) },
            |v| integer_sort(v, 0, range as i32),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(
    small_range_few_numbers,
    sort_small_few,
    integer_sort_small_few
);
criterion_group!(
    large_range_few_numbers,
    sort_large_few,
    integer_sort_large_few
);
criterion_group!(
    small_range_many_numbers,
    sort_small_many,
    integer_sort_small_many
);
criterion_group!(
    large_range_many_numbers,
    sort_large_many,
    integer_sort_large_many
);
criterion_main!(
    small_range_few_numbers,
    large_range_few_numbers,
    small_range_many_numbers,
    large_range_many_numbers
);
