use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main, Throughput};

use rust_collections_performance::{
    add_btree, add_hashset, add_heap, add_linked_list, add_vec, add_vec_capacity,
};

pub fn benchmark_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");
    let k = 1_00u64;
    for size in 1..=3 {
        let size = k.pow(size);
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, size| {
            b.iter_batched(
                || (0..*size).collect::<Vec<_>>(),
                |items| add_vec(&items[..]),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("vec_capacity", size), &size, |b, size| {
            b.iter_batched(
                || (0..*size).collect::<Vec<_>>(),
                |items| add_vec_capacity(&items[..]),
                BatchSize::LargeInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("hashset", size), &size, |b, size| {
            b.iter_batched(
                || (0..*size).collect::<Vec<_>>(),
                |items| add_hashset(&items[..]),
                BatchSize::LargeInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("btree", size), &size, |b, size| {
            b.iter_batched(
                || (0..*size).collect::<Vec<_>>(),
                |items| add_btree(&items[..]),
                BatchSize::LargeInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("heap", size), &size, |b, size| {
            b.iter_batched(
                || (0..*size).collect::<Vec<_>>(),
                |items| add_heap(&items[..]),
                BatchSize::LargeInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("linked list", size), &size, |b, size| {
            b.iter_batched(
                || (0..*size).collect::<Vec<_>>(),
                |items| add_linked_list(&items[..]),
                BatchSize::LargeInput,
            )
        });
    }
}


pub fn benchmark_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");
    let k = 1_00u64;
    for size in 1..=3 {
        let size = k.pow(size);
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, size| {
            b.iter_batched(
                || add_vec(&(0..*size).collect::<Vec<_>>()),
                |container| container.contains(&1337),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("vec_capacity", size), &size, |b, size| {
            b.iter_batched(
                || add_vec_capacity(&(0..*size).collect::<Vec<_>>()),
                |container| container.contains(&1337),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("hashset", size), &size, |b, size| {
            b.iter_batched(
                || add_hashset(&(0..*size).collect::<Vec<_>>()),
                |container| container.contains(&1337),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("btree", size), &size, |b, size| {
            b.iter_batched(
                || add_btree(&(0..*size).collect::<Vec<_>>()),
                |container| container.contains_key(&1337),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("heap", size), &size, |b, size| {
            b.iter_batched(
                || add_heap(&(0..*size).collect::<Vec<_>>()),
                |container| container.into_iter().find(|x| *x == 1337),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("linked_list", size), &size, |b, size| {
            b.iter_batched(
                || add_linked_list(&(0..*size).collect::<Vec<_>>()),
                |container| container.contains(&1337),
                BatchSize::LargeInput,
            );
        });
    }
}

criterion_group!(
    benches,
    benchmark_add,
    benchmark_contains
);
criterion_main!(add, benches);
