use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use detect_anagram_performance::{anagram_array, anagram_map, anagram_sort};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("anagram");
    // for alpha in [10, 100, 1000] {
    for beta in [10, 100, 1000].iter() {
        let mut alphabet = Vec::new();
        for n in 1..*beta {
            alphabet.push(char::from_u32(n).unwrap_or(' '));
        }
        let alphabet: String = alphabet.into_iter().collect();
        let a = random_string::generate(20, &alphabet);
        let b = random_string::generate(20, &alphabet);
        group.bench_function(BenchmarkId::new("sort", beta), |t| t.iter(|| anagram_sort(&a, &b)));
        group.bench_function(BenchmarkId::new("array", beta), |t| t.iter(|| anagram_array(&a, &b, *beta as usize)));
        group.bench_function(BenchmarkId::new("map", beta), |t| t.iter(|| anagram_map(&a, &b)));
    }
    // }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);