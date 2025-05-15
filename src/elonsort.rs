use crate::visualizer;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn elon_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n == 0 {
        return;
    }

    let mut rng = thread_rng();
    // Determine the number of elements to keep (delete half the array).
    // When n is odd, this keeps one more element than deleted.
    let keep_count = n - n / 2;

    // Create a list of indices for the original array and shuffle them.
    let mut indices: Vec<usize> = (0..n).collect();
    indices.shuffle(&mut rng);

    // Retain only the first keep_count indices.
    indices.truncate(keep_count);
    // Sort indices to preserve the relative original order.
    indices.sort_unstable();

    // Move the kept elements to the start of the array.
    for (new_idx, &orig_idx) in indices.iter().enumerate() {
        arr[new_idx] = arr[orig_idx];
    }

    // Visualize the final "sorted" state.
    visualizer::draw(&arr[..keep_count]);
}