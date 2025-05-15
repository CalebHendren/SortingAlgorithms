use crate::visualizer;

/// Performs counting sort on a mutable slice of i32 numbers.
///
/// Counting sort is an efficient sorting algorithm for arrays with a small range of integer values.
/// It works by counting the number of occurrences of each unique value in the input slice, and then
/// calculating the starting index of each key in the output sorted array.
pub fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    // Determine the minimum and maximum values in the array.
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();

    // Create a count array with a slot for each value in the range [min, max].
    let range = (max - min + 1) as usize;
    let mut counts = vec![0; range];

    // Count the occurrences of each value.
    for &num in arr.iter() {
        counts[(num - min) as usize] += 1;
    }

    // Reconstruct the sorted array.
    let mut index = 0;
    for i in 0..range {
        for _ in 0..counts[i] {
            arr[index] = i as i32 + min;
            index += 1;
            visualizer::draw(arr);
        }
    }
}