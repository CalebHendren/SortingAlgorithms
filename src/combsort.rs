use crate::visualizer;

/// Performs comb sort on a mutable slice of i32 numbers.
///
/// Comb sort is an improvement over bubble sort. It works by comparing elements
/// that are a certain gap apart and gradually reducing the gap until it becomes 1.
pub fn comb_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    let shrink_factor = 1.3;
    let mut gap = n;
    let mut sorted = false;

    while gap > 1 || !sorted {
        gap = (gap as f64 / shrink_factor).floor() as usize;
        if gap < 1 {
            gap = 1;
        }
        sorted = true;
        for i in 0..n - gap {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
                sorted = false;
                visualizer::draw(arr);
            }
        }
    }
}