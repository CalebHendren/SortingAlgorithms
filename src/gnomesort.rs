use crate::visualizer;

/// Performs gnome sort on a mutable slice of i32 numbers.
///
/// Gnome sort is a simple sorting algorithm that works by moving an element to its proper position
/// by comparing it with the previous element and swapping if necessary, similar in spirit to insertion sort.
pub fn gnome_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    
    let mut index = 0;
    while index < n {
        if index == 0 || arr[index] >= arr[index - 1] {
            index += 1;
        } else {
            arr.swap(index, index - 1);
            visualizer::draw(arr);
            index -= 1;
        }
    }
}