use rand::prelude::*;
use std::collections::HashSet;

pub fn generate_random_array(
    min: i32,
    max: i32,
    count: usize,
    allow_duplicates: bool,
) -> Vec<i32> {
    let mut rng = thread_rng();

    if allow_duplicates {
        (0..count)
            .map(|_| rng.gen_range(min..=max))
            .collect()
    } else {
        let range_size = (max - min + 1) as usize;
        assert!(
            count <= range_size,
            "Cannot generate {} unique values in range {}..={}",
            count, min, max
        );
        let mut set = HashSet::new();
        while set.len() < count {
            set.insert(rng.gen_range(min..=max));
        }
        set.into_iter().collect()
    }
}

/// Generate with default settings: min=0, max=100, count=100, allow_duplicates=true
pub fn generate_default_array() -> Vec<i32> {
    generate_random_array(0, 100, 100, false)
}
