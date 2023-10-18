use std::collections::HashMap;

pub fn anagram_sort(a: &str, b: &str) -> bool {
    let mut a = a.chars().collect::<Vec<char>>();
    a.sort();

    let mut b = b.chars().collect::<Vec<char>>();
    b.sort();

    a == b
}

pub fn anagram_array(a: &str, b: &str, n: usize) -> bool {
    let mut a_count = vec![0; n];
    for c in a.chars() {
        a_count[c as usize] += 1;
    }

    let mut b_count = vec![0; n];
    for c in b.chars() {
        b_count[c as usize] += 1;
    }

    a_count == b_count
}

pub fn anagram_map(a: &str, b: &str) -> bool {
    let mut a_count = HashMap::<usize, u8>::with_capacity(100);
    for c in a.chars() {
        *a_count.entry(c as usize).or_default() += 1;
    }
    let mut b_count = HashMap::<usize, u8>::with_capacity(100);
    for c in b.chars() {
        *b_count.entry(c as usize).or_default() += 1;
    }

    a_count == b_count
}


mod test {
    #[test]
    fn is_anagram() {
        let a = "elegant man";
        let b = "a gentleman";
        debug_assert!(crate::anagram_sort(a, b));
        debug_assert!(crate::anagram_array(a, b, 128));
        debug_assert!(crate::anagram_map(a, b));
    }

    #[test]
    fn not_anagram() {
        let a = "elegant man";
        let b = "muppet";
        debug_assert!(!crate::anagram_sort(a, b));
        debug_assert!(!crate::anagram_array(a, b, 128));
        debug_assert!(!crate::anagram_map(a, b));
    }
}

