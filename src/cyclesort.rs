use crate::visualizer;

/// Performs cycle sort on a mutable slice of i32 numbers.
///
/// Cycle sort is an in-place, unstable sorting algorithm that minimizes the number of writes to the array.
/// It determines the correct position for each element and rotates the cycle until complete.
pub fn cycle_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    for cycle_start in 0..n - 1 {
        let mut item = arr[cycle_start];
        let mut pos = cycle_start;
        
        // Find the correct position where the item should go.
        for i in cycle_start + 1..n {
            if arr[i] < item {
                pos += 1;
            }
        }
        
        // If the item is already in the right position, move to the next.
        if pos == cycle_start {
            continue;
        }
        
        // Skip over any duplicate elements.
        while item == arr[pos] {
            pos += 1;
        }
        
        // Place the item in its correct position.
        if pos != cycle_start {
            item = std::mem::replace(&mut arr[pos], item);
            visualizer::draw(arr);
        }
        
        // Rotate the rest of the cycle.
        while pos != cycle_start {
            pos = cycle_start;
            
            // Determine the position for the new item.
            for i in cycle_start + 1..n {
                if arr[i] < item {
                    pos += 1;
                }
            }
            
            // Skip over duplicates.
            while item == arr[pos] {
                pos += 1;
            }
            
            // Place the item in its correct position.
            if item != arr[pos] {
                item = std::mem::replace(&mut arr[pos], item);
                visualizer::draw(arr);
            }
        }
    }
}