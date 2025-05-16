/// Performs cocktail shaker sort on a mutable slice of i32 numbers.
/// 
/// This function sorts the slice in ascending order using a bidirectional bubble sort.
/// It updates the visualization after each swap using the visualizer crate.
use crate::visualizer;

pub fn cocktail_shaker_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    
    let mut start = 0;
    let mut end = n - 1;
    
    while start < end {
        let mut swapped = false;
        
        // Forward pass: bubble up the largest element to the end.
        for i in start..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
                visualizer::draw(arr);
            }
        }
        if !swapped {
            break;
        }
        
        end -= 1;
        swapped = false;
        
        // Backward pass: bubble down the smallest element to the beginning.
        for i in (start..end).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
                visualizer::draw(arr);
            }
        }
        if !swapped {
            break;
        }
        
        start += 1;
    }
}