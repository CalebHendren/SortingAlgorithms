use crate::visualizer;
use rand::prelude::*;

/// Performs bozo sort on a mutable slice of i32 numbers.
///
/// Bozo sort repeatedly selects two random indices and swaps their elements until the array is sorted.
/// This algorithm is highly inefficient and is generally used for educational or entertainment purposes.
pub fn bozo_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    
    let mut rng = thread_rng();
    
    // Continue swapping random pairs until the array is sorted.
    while !is_sorted(arr) {
        let i = rng.gen_range(0..len);
        let j = rng.gen_range(0..len);
        
        if i != j {
            arr.swap(i, j);
            visualizer::draw(arr);
        }
    }
    
    // Final visualization after sorting is complete.
    visualizer::draw(arr);
}

/// Returns true if the array is sorted in non-decreasing order.
fn is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}