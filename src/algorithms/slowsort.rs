use crate::visualizer;

/// Performs slowsort on a mutable slice of i32 numbers.
///
/// Slowsort is a highly inefficient sorting algorithm based on a recursive, divide-and-conquer strategy.
/// It recursively sorts the two halves of the array, swaps the last element with the middle element if needed,
/// and finally re-sorts the array excluding the last element.
pub fn slowsort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    slowsort_rec(arr, 0, arr.len() - 1);
    // Final visualization after sorting is complete.
    visualizer::draw(arr);
}

/// Recursive helper function for slowsort.
/// It works on the subarray of arr defined by the indices [i, j].
fn slowsort_rec(arr: &mut [i32], i: usize, j: usize) {
    // Base case: when the subarray has one or no element.
    if i >= j {
        return;
    }

    // Find the middle index.
    let m = (i + j) / 2;

    // Recursively sort the first half.
    slowsort_rec(arr, i, m);
    // Recursively sort the second half.
    slowsort_rec(arr, m + 1, j);

    // If the last element is less than the middle element, swap them.
    if arr[j] < arr[m] {
        arr.swap(j, m);
        visualizer::draw(arr);
    }

    // Recursively re-sort the array excluding the last element.
    // Since j > i from the base check, j is at least 1 so j - 1 is valid.
    slowsort_rec(arr, i, j - 1);
}