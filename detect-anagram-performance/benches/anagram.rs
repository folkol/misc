use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use detect_anagram_performance::{anagram_array, anagram_map, anagram_sort};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("anagram");
    for alphabet_size in [100, 1000, 10000].iter() {
        // for alphabet_size in [10, 1000].iter() {
        let mut alphabet = Vec::new();
        for n in 1..*alphabet_size {
            alphabet.push(char::from_u32(n).or_else(|| {
                Some(32 as char)
            }).unwrap());
        }
        let alphabet: String = alphabet.into_iter().collect();
        let a = random_string::generate(20, &alphabet);
        let b = random_string::generate(20, &alphabet);
        group.bench_with_input(BenchmarkId::new("sorting", alphabet_size), alphabet_size,
                               |bencher, &i| bencher.iter(|| anagram_sort(&a, &b)));
        group.bench_with_input(BenchmarkId::new("array", alphabet_size), alphabet_size,
                               |bencher, &i| bencher.iter(|| anagram_array(&a, &b, *alphabet_size as usize)));
        group.bench_with_input(BenchmarkId::new("map", alphabet_size), alphabet_size,
                               |bencher, &i| bencher.iter(|| anagram_map(&a, &b)));
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);