use crate::visualizer;

pub fn brick_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut is_sorted = false;

    while !is_sorted {
        is_sorted = true;

        // Odd indexed elements - parallel processing
        let odd_swaps: Vec<_> = (1..n - 1)
            .step_by(2)
            .filter(|&i| arr[i] > arr[i + 1])
            .collect();
        for &i in &odd_swaps {
            arr.swap(i, i + 1);
            visualizer::draw(arr);
        }
        if !odd_swaps.is_empty() {
            is_sorted = false;
        }

        // Even indexed elements - parallel processing
        let even_swaps: Vec<_> = (0..n - 1)
            .step_by(2)
            .filter(|&i| arr[i] > arr[i + 1])
            .collect();
        for &i in &even_swaps {
            arr.swap(i, i + 1);
            visualizer::draw(arr);
        }
        if !even_swaps.is_empty() {
            is_sorted = false;
        }
    }
}