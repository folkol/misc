use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};

use rust_collections_performance::{
    add_btree, add_hashset, add_heap, add_linked_list, add_vec, add_vec_capacity,
};

macro_rules! test_add {
    ($name: literal, $func: ident, $size: ident, $group: ident) => {
        $group.bench_with_input(BenchmarkId::new($name, $size), &$size, |b, $size| {
            b.iter_batched(
                || (0..*$size).collect::<Vec<_>>(),
                |items| $func(black_box(&items[..])),
                BatchSize::SmallInput,
            );
        });
    };
}

macro_rules! test_contains {
    ($name: literal, $func: ident, $size: ident, $group: ident) => {
        let last_value = $size - 1;
        $group.bench_with_input(BenchmarkId::new($name, $size), &$size, |b, $size| {
            b.iter_batched(
                || $func(&(0..*$size).collect::<Vec<_>>()),
                |container| container.contains(black_box(&last_value)),
                BatchSize::SmallInput,
            );
        });
    };
}

pub fn benchmark_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");
    let k = 1_00u64;
    for size in 1..=3 {
        let size = k.pow(size);
        group.throughput(Throughput::Elements(size));

        test_add!("vec", add_vec, size, group);
        test_add!("vec_capacity", add_vec_capacity, size, group);
        test_add!("hashset", add_hashset, size, group);
        test_add!("btree", add_btree, size, group);
        test_add!("heap", add_heap, size, group);
        test_add!("linked list", add_linked_list, size, group);
    }
}

pub fn benchmark_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");
    let k = 1_00u64;
    for size in 1..=3 {
        let size = k.pow(size);
        group.throughput(Throughput::Elements(size));
        test_contains!("vec", add_vec, size, group);
        test_contains!("vec capacity", add_vec_capacity, size, group);
        test_contains!("hashset", add_hashset, size, group);
        let last_value = size - 1;
        group.bench_with_input(BenchmarkId::new("btree", size), &size, |b, size| {
            b.iter_batched(
                || add_btree(&(0..*size).collect::<Vec<_>>()),
                |container| container.contains_key(black_box(&last_value)), // missing
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("heap", size), &size, |b, size| {
            b.iter_batched(
                || add_heap(&(0..*size).collect::<Vec<_>>()),
                |container| container.into_iter().find(|x| *x == last_value),
                BatchSize::LargeInput,
            );
        });
        test_contains!("linked list", add_linked_list, size, group);
    }
}

criterion_group!(benches, benchmark_add, benchmark_contains);
criterion_main!(benches);
