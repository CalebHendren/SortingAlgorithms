use crate::visualizer;

/// Performs Stalin sort on a mutable slice of i32 numbers.
///
/// In this sort any element that is not in non-decreasing order is deleted
/// from the sequence. The remaining elements form a sorted subsequence.
/// The function visualizes the state of the kept elements after each insertion or deletion.
pub fn stalinsort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    // The first element is always kept.
    let mut kept_length = 1;
    visualizer::draw(&arr[0..kept_length]);

    // Iterate through the rest of the array.
    for i in 1..arr.len() {
        if arr[i] >= arr[kept_length - 1] {
            // Element is in order and kept.
            arr[kept_length] = arr[i];
            kept_length += 1;
            visualizer::draw(&arr[0..kept_length]);
        } else {
            // Element is not in order and is "deleted" (skipped).
            visualizer::draw(&arr[0..kept_length]);
        }
    }
}