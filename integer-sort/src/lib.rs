#[inline]
pub fn sort(mut n: Vec<u64>) -> Vec<u64> {
    n.sort();
    n
}

#[inline]
pub fn integer_sort(mut xs: Vec<u64>, min: i32, max: i32) -> Vec<u64> {
    let min = min as u64;
    let max = max as u64;
    let mut counter = vec![0; (max - min + 1) as usize];
    // println!("counter_size {}", counter.len());
    for x in &xs {
        counter[(*x - min) as usize] += 1;
    }

    xs.clear();
    for (i, n) in counter.iter().enumerate() {
        for _ in 0..*n {
            xs.push(i as u64 + min);
        }
    }
    xs
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        let xs = vec![10, 50, 20, 90];
        let xs = crate::sort(xs);
        assert_eq!(xs, vec![10, 20, 50, 90]);
    }

    #[test]
    fn integer_sort() {
        let xs = vec![10, 50, 20, 90];
        let xs = crate::integer_sort(xs, 10, 90);
        assert_eq!(xs, vec![10, 20, 50, 90]);
    }
}
