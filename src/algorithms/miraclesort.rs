use crate::visualizer;

/// Performs miracle sort on a mutable slice of i32 numbers.
///
/// This algorithm constantly checks if the array is sorted without doing any actual sorting.
/// It relies on the miracle that, if you wait long enough, the array might sort itself.
pub fn miracle_sort(arr: &mut [i32]) {
    // Continuously check if the array is already sorted.
    while !is_sorted(arr) {
        // Visualize the current state.
        visualizer::draw(arr);
        // Optionally, a brief pause could be inserted here to avoid a tight loop.
    }
    // Final visualization once the array is sorted.
    visualizer::draw(arr);
}

/// Helper function that returns true if the array is sorted in non-decreasing order.
fn is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}